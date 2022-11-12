pub mod ast_parser {
    use super::formater::refactor;
    use crate::lexer::tokenizer::*;
    pub fn generate_ast(source_path: &str) -> Option<Vec<Head>> {
        use std::fs;
        let source =
            fs::read_to_string(source_path).expect("Unexpected problem while opening AST file");
        let (tokens, mut lines, mut errors) = parse(source, false);
        if let Ok(mut refactored) = refactor(tokens, &mut lines, &mut errors) {
            return Some(analize_tree(&mut refactored));
        } else {
            println!(
                "Could not parse AST {source_path}, number of errors: {}",
                errors.len()
            );
            return None;
        }
    }
    pub fn analize_input(nodes: &Vec<Head>, lines: Vec<(usize, usize)>, ast: Vec<Head>){
        
    }
    fn analize_tree(tokens: &mut Vec<Tokens>) -> Vec<Head> {
        let mut tree: Vec<Head> = vec![];
        let mut idx = 0;
        while let Some(head) = read_head(tokens, &mut idx) {
            tree.push(head);
            idx += 1;
        }
        tree
    }
    fn read_head(tokens: &mut Vec<Tokens>, idx: &mut usize) -> Option<Head> {
        if tokens.len() == *idx {
            return None;
        }
        let name = if let Tokens::Text(txt) = &tokens[*idx] {
            *idx += 1;
            txt.to_string()
        } else {
            return None;
        };
        let mut parameters = vec![];
        while tokens[*idx] != Tokens::Tab {
            match &tokens[*idx] {
                Tokens::SquareBracket(closed) => {
                    if !closed {
                        if let Tokens::Text(txt) = &tokens[*idx + 1] {
                            parameters.push(HeadParam::Array(txt.to_string()));
                            *idx += 2;
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                Tokens::Text(txt) => {
                    parameters.push(HeadParam::Value(txt.to_string()));
                    *idx += 1;
                }
                _ => {}
            }
        }
        Some(Head {
            name,
            parameters,
            nodes: get_nodes(tokens, idx, 1),
        })
    }
    fn get_nodes(tokens: &mut Vec<Tokens>, idx: &mut usize, tabs: usize) -> Vec<NodeType> {
        let mut nodes: Vec<NodeType> = vec![];
        while count_tabs(&tokens, *idx) == tabs {
            *idx += tabs;
            match tokens[*idx + 1] {
                Tokens::Optional => {
                    tokens.remove(*idx + 1);
                    let node = NodeType::Maybe(Node {
                        kind: tokens.remove(*idx),
                        arguments: get_node_args(&tokens, idx),
                        nodes: get_nodes(tokens, idx, tabs + 1),
                    });
                    nodes.push(node);
                }
                Tokens::Operator(op) => match op {
                    Operators::Not => {
                        tokens.remove(*idx + 1);
                        let node = NodeType::Command(Node {
                            kind: tokens.remove(*idx),
                            arguments: get_node_args(&tokens, idx),
                            nodes: get_nodes(tokens, idx, tabs + 1),
                        });
                        nodes.push(node);
                    }
                    Operators::Equal => {
                        let node = NodeType::ArgsCondition(ArgsCon {
                            params: get_node_args(&tokens, idx),
                            nodes: get_nodes(tokens, idx, tabs + 1),
                        });
                        nodes.push(node);
                    }
                    _ => {}
                },
                _ => {
                    let node = NodeType::Expect(Node {
                        kind: tokens.remove(*idx),
                        arguments: get_node_args(&tokens, idx),
                        nodes: get_nodes(tokens, idx, tabs + 1),
                    });
                    nodes.push(node);
                }
            }
        }
        nodes
    }
    fn get_node_args(tokens: &Vec<Tokens>, idx: &mut usize) -> Vec<NodeParam> {
        let mut args = vec![];
        while let Tokens::Text(name) = &tokens[*idx] {
            *idx += 2;
            if let Tokens::String(value) = &tokens[*idx] {
                args.push(NodeParam {
                    name: name.to_string(),
                    value: value.to_string(),
                })
            }
            *idx += 1;
        }
        args
    }
    fn count_tabs(tokens: &Vec<Tokens>, idx: usize) -> usize {
        let mut count = 0;
        while let Tokens::Tab = &tokens[idx + count] {
            count += 1;
        }
        count
    }
    struct Output {
        
    }
    #[derive(Debug)]
    pub struct Head {
        name: String,
        parameters: Vec<HeadParam>,
        nodes: Vec<NodeType>,
    }
    #[derive(Debug)]
    pub enum HeadParam {
        Array(String),
        Value(String),
    }
    #[derive(Debug)]
    pub struct Node {
        kind: Tokens,
        arguments: Vec<NodeParam>,
        nodes: Vec<NodeType>,
    }
    #[derive(Debug)]
    pub enum NodeType {
        Maybe(Node),
        Expect(Node),
        Command(Node),
        ArgsCondition(ArgsCon),
    }
    #[derive(Debug)]
    pub struct NodeParam {
        name: String,
        value: String,
    }
    #[derive(Debug)]
    pub struct ArgsCon {
        params: Vec<NodeParam>,
        nodes: Vec<NodeType>,
    }
}

mod formater {
    use crate::{
        lexer::tokenizer::{/*Keywords,*/ deparse_token, Operators, Tokens},
        token_refactor::{parse_err::Errors, refactorer::LexingErr},
    };

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
                // nesting
                for char in txt.chars() {
                    if !char.is_whitespace() {
                        return 1;
                    }
                }
                if txt == "\t" {
                    tokens[idx] = Tokens::Tab;
                    return 1;
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
            Tokens::AngleBracket(bol) => {
                if let Tokens::Operator(eq) = tokens[idx + 1] {
                    if let Operators::Equal = eq {
                        tokens[idx] = match *bol {
                            true => Tokens::Operator(Operators::LessEq),
                            false => Tokens::Operator(Operators::MoreEq),
                        };
                        tokens.remove(idx + 1);
                        lines.remove(idx + 1);
                    }
                }
            }
            _ => {}
        }
        1
    }
}
