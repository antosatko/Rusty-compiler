pub mod refactorer {
    use crate::lexer::{
        self,
        compiler_data::{Operators, Tokens},
    };
    pub fn refactor(mut tokens: Vec<Tokens>) -> Result<Vec<Tokens>, LexingErr> {
        let mut i = 0;
        while i < tokens.len() {
            i += process_token(&mut tokens, i);
        }
        Ok(tokens)
    }
    fn process_token(tokens: &mut Vec<Tokens>, idx: usize) -> usize {
        match &tokens[idx] {
            Tokens::DoubleQuotes => {
                let mut i = idx + 1;
                let mut res = String::new();
                while tokens[i] != Tokens::DoubleQuotes {
                    res.push_str(&format!("{:?}", tokens[i]));
                    i += 1;
                }
                tokens.splice(idx + 1..i + 1, []);
                tokens[idx] = Tokens::String(res);
            }
            Tokens::Space => {
                tokens.remove(idx);
                return 0;
            }
            Tokens::Colon => {
                if let Tokens::Colon = tokens[idx + 1] {
                    tokens[idx] = Tokens::DoubleColon;
                    tokens.remove(idx + 1);
                }
            }
            Tokens::Semicolon => {
                while let Tokens::Semicolon = tokens[idx + 1] {
                    tokens.remove(idx + 1);
                }
            }
            Tokens::Text(txt) => {
                for char in txt.chars() {
                    if !char.is_whitespace() {
                        return 1;
                    }
                }
                tokens.remove(idx);
                return 0;
            }
            Tokens::Operator(op) => match op {
                Operators::Add => {
                    if let Tokens::Operator(Operators::Equal) = tokens[idx + 1] {
                        tokens[idx] = Tokens::Operator(Operators::AddEq);
                        tokens.remove(idx + 1);
                    }
                }
                Operators::Sub => {
                    if let Tokens::Operator(Operators::Equal) = tokens[idx + 1] {
                        tokens[idx] = Tokens::Operator(Operators::SubEq);
                        tokens.remove(idx + 1);
                    }
                }
                Operators::Mul => {
                    if let Tokens::Operator(Operators::Equal) = tokens[idx + 1] {
                        tokens[idx] = Tokens::Operator(Operators::MulEq);
                        tokens.remove(idx + 1);
                    }
                }
                Operators::Div => {
                    if let Tokens::Operator(Operators::Equal) = tokens[idx + 1] {
                        tokens[idx] = Tokens::Operator(Operators::DivEq);
                        tokens.remove(idx + 1);
                    }
                }
                Operators::Not => {
                    if let Tokens::Operator(Operators::Equal) = tokens[idx + 1] {
                        tokens[idx] = Tokens::Operator(Operators::NotEqual);
                        tokens.remove(idx + 1);
                    }
                }
                Operators::Equal => {
                    if let Tokens::Operator(Operators::Equal) = tokens[idx + 1] {
                        tokens[idx] = Tokens::Operator(Operators::DoubleEq);
                        tokens.remove(idx + 1);
                    }
                }
                _ => {}
            },
            _ => {}
        }
        1
    }
    pub enum LexingErr {}
}
