pub mod refactorer {
    use crate::lexer::{
        compiler::deparse_token,
        compiler_data::{Keywords, Operators, Tokens},
    };

    use super::parse_err::Errors;
    pub fn refactor(
        mut tokens: Vec<Tokens>,
        lines: &mut Vec<(usize, usize)>,
        errors: &mut Vec<Errors>,
    ) -> Result<Vec<Tokens>, LexingErr> {
        let mut i = 0;
        while i < tokens.len() {
            i += process_token(&mut tokens, i, lines, errors);
        }
        Ok(tokens)
    }
    fn process_token(
        tokens: &mut Vec<Tokens>,
        idx: usize,
        lines: &mut Vec<(usize, usize)>,
        errors: &mut Vec<Errors>,
    ) -> usize {
        match &tokens[idx] {
            Tokens::DoubleQuotes => {
                let mut i = idx + 1;
                let mut res = String::new();
                while tokens[i] != Tokens::DoubleQuotes {
                    res.push_str(&deparse_token(&tokens[i]));
                    i += 1;
                    if i == tokens.len() {
                        // syntax err: end of string never found
                        tokens.splice(idx + 1.., []);
                        lines.splice(idx + 1.., []);
                        tokens[idx] = Tokens::String(res);
                        return 1;
                    }
                }
                tokens.splice(idx + 1..i + 1, []);
                lines.splice(idx + 1..i + 1, []);
                tokens[idx] = Tokens::String(res);
            }
            Tokens::Space => {
                tokens.remove(idx);
                lines.remove(idx);
                return 0;
            }
            Tokens::Colon => {
                if let Tokens::Colon = tokens[idx + 1] {
                    tokens[idx] = Tokens::DoubleColon;
                    tokens.remove(idx + 1);
                    lines.remove(idx + 1);
                }
            }
            Tokens::Semicolon => {
                while let Tokens::Semicolon = tokens[idx + 1] {
                    tokens.remove(idx + 1);
                    lines.remove(idx + 1);
                }
            }
            Tokens::Text(txt) => {
                let bytes = txt.as_bytes();
                if let Some(first) = bytes.get(0) {
                    if first.is_ascii_digit() {
                        // float
                        if let Tokens::Dot = &tokens[idx + 1] {
                            let first_num = if let Ok(num) = txt.parse::<usize>() {
                                num
                            } else {
                                // syntax err: incorrect number
                                errors.push(Errors::InvalidNumber(lines[idx], txt.to_string()));
                                return 1;
                            };
                            if let Tokens::Text(txt2) = &tokens[idx + 2] {
                                if let Ok(num2) = txt2.parse::<usize>() {
                                    tokens[idx] = Tokens::Number(first_num, num2, 'f');
                                    tokens.remove(idx + 1);
                                    tokens.remove(idx + 1);
                                    lines.remove(idx + 1);
                                    lines.remove(idx + 1);
                                } else {
                                    // syntax err: incorrect number
                                    let mut res = txt.to_string();
                                    res.push('.');
                                    res.push_str(txt2);
                                    errors.push(Errors::InvalidNumber(lines[idx], res));
                                    return 1;
                                };
                            } else {
                                // syntax err: unexpected symbol: .
                            }
                        // int
                        } else {
                            if bytes[bytes.len() - 1].is_ascii_digit() {
                                if let Ok(num) = txt.parse::<usize>() {
                                    tokens[idx] = Tokens::Number(num, 0, 'i')
                                } else {
                                    errors.push(Errors::InvalidNumber(lines[idx], txt.to_string()));
                                    // syntax err: incorrect number
                                }
                            } else {
                                if let Ok(num) = txt[..txt.len() - 1].parse::<usize>() {
                                    tokens[idx] =
                                        Tokens::Number(num, 0, bytes[bytes.len() - 1] as char)
                                } else {
                                    errors.push(Errors::InvalidNumber(lines[idx], txt.to_string()));
                                    // syntax err: incorrect number
                                }
                            }
                        }
                        return 1;
                    }
                }
                for char in txt.chars() {
                    if !char.is_whitespace() {
                        return 1;
                    }
                }
                lines.remove(idx);
                tokens.remove(idx);
                return 0;
            }
            Tokens::Operator(op) => match op {
                Operators::Add => {
                    if let Tokens::Operator(Operators::Equal) = tokens[idx + 1] {
                        tokens[idx] = Tokens::Operator(Operators::AddEq);
                        tokens.remove(idx + 1);
                        lines.remove(idx + 1);
                    }
                }
                Operators::Sub => {
                    if let Tokens::Operator(Operators::Equal) = tokens[idx + 1] {
                        tokens[idx] = Tokens::Operator(Operators::SubEq);
                        tokens.remove(idx + 1);
                        lines.remove(idx + 1);
                    }
                }
                Operators::Mul => {
                    if let Tokens::Operator(Operators::Equal) = tokens[idx + 1] {
                        tokens[idx] = Tokens::Operator(Operators::MulEq);
                        tokens.remove(idx + 1);
                        lines.remove(idx + 1);
                    }
                }
                Operators::Div => {
                    if let Tokens::Operator(Operators::Equal) = tokens[idx + 1] {
                        tokens[idx] = Tokens::Operator(Operators::DivEq);
                        tokens.remove(idx + 1);
                        lines.remove(idx + 1);
                    } else if let Tokens::Operator(Operators::Div) = tokens[idx + 1] {
                        tokens[idx] = Tokens::Keyword(Keywords::CommentLine);
                        tokens.remove(idx + 1);
                        lines.remove(idx + 1);
                        return 0;
                    } else if let Tokens::Operator(Operators::Mul) = tokens[idx + 1] {
                        tokens[idx] = Tokens::Keyword(Keywords::CommentBlock);
                        tokens.remove(idx + 1);
                        lines.remove(idx + 1);
                        return 0;
                    }
                }
                Operators::Not => {
                    if let Tokens::Operator(Operators::Equal) = tokens[idx + 1] {
                        tokens[idx] = Tokens::Operator(Operators::NotEqual);
                        tokens.remove(idx + 1);
                        lines.remove(idx + 1);
                    }
                }
                Operators::Equal => {
                    if let Tokens::Operator(Operators::Equal) = tokens[idx + 1] {
                        tokens[idx] = Tokens::Operator(Operators::DoubleEq);
                        tokens.remove(idx + 1);
                        lines.remove(idx + 1);
                    }
                }
                _ => {}
            },
            Tokens::Keyword(kw) => match kw {
                Keywords::CommentLine => {
                    loop {
                        tokens.remove(idx);
                        lines.remove(idx);
                        if let Tokens::Text(str) = &tokens[idx] {
                            if str == "\n" {
                                break;
                            }
                        }
                    }
                    tokens.remove(idx);
                    lines.remove(idx);
                    return 0;
                }
                Keywords::CommentBlock => {
                    loop {
                        tokens.remove(idx);
                        lines.remove(idx);
                        if let Tokens::Operator(Operators::Mul) = &tokens[idx] {
                            if let Tokens::Operator(Operators::Div) = &tokens[idx + 1] {
                                break;
                            }
                        }
                    }
                    tokens.remove(idx);
                    lines.remove(idx);
                    tokens.remove(idx);
                    lines.remove(idx);
                    return 0;
                }
                _ => {}
            },
            _ => {}
        }
        1
    }
    pub enum LexingErr {}
}

pub mod parse_err {
    use crate::lexer::compiler_data::Tokens;

    pub enum Errors {
        // (line, column) token
        UnexpectedToken((usize, usize), Tokens),
        // (line, column) number
        InvalidNumber((usize, usize), String),
    }
}
