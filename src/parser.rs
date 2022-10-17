pub mod syntax {
    use std::{collections::HashMap, vec};

    use crate::lexer::compiler_data::*;
    pub fn tokenize(file: String) {
        let parse_rules = get_rules();
    }
    pub fn get_token_block(kind: Kinds, tokens: &mut Vec<Tokens>) -> Result<Vec<Tokens>, ParseErr> {
        match kind {
            Kinds::Block => {
                let mut cur_brackets = 1;
                let mut i = 1;
                if let Tokens::CurlyBracket(bol) = tokens[0] {
                    if bol {
                        return Err(ParseErr::UnexpectedToken);
                        // syntax err: expected "{" at the start of code block, found "}"
                    }
                } else {
                    return Err(ParseErr::UnexpectedToken);
                    // syntax err: expected "{" at the start of code block
                }
                loop {
                    if i == tokens.len() {
                        return Err(ParseErr::FileEnd);
                        // syntax err: end of block never found
                    }
                    if let Tokens::CurlyBracket(bol) = tokens[i] {
                        cur_brackets += if bol { -1 } else { 1 }
                    }
                    if cur_brackets == 0 {
                        return Ok(tokens.drain(..i).collect());
                    }
                    i += 1;
                }
                return Err(ParseErr::FileEnd);
            }
            Kinds::Token(token) => {}
            Kinds::Value => {
                let mut i = 0;
                loop {
                    if i == tokens.len() {
                        return Err(ParseErr::FileEnd);
                    }
                    let mut brackets = (0, 0, 0, 0);
                    match tokens[i] {
                        Tokens::Parenteses(bol) => {
                            if bol {
                                brackets.0 += 1;
                                if brackets.0 == -1 {
                                    break;
                                }
                            } else {
                                brackets.0 -= 1;
                            }
                        }
                        Tokens::AngleBracket(bol) => {
                            if bol {
                                brackets.1 += 1;
                                if brackets.1 == -1 {
                                    break;
                                }
                            } else {
                                brackets.1 -= 1;
                            }
                        }
                        Tokens::CurlyBracket(bol) => {
                            if bol {
                                brackets.2 += 1;
                                if brackets.2 == -1 {
                                    break;
                                }
                            } else {
                                brackets.2 -= 1;
                            }
                        }
                        Tokens::SquareBracket(bol) => {
                            if bol {
                                brackets.3 += 1;
                                if brackets.3 == -1 {
                                    break;
                                }
                            } else {
                                brackets.3 -= 1;
                            }
                        }
                        Tokens::Semicolon => {
                            if brackets.0 == 0
                                && brackets.1 == 0
                                && brackets.2 == 0
                                && brackets.3 == 0
                            {
                                break;
                            }
                        }
                        Tokens::Colon => {
                            if brackets.0 == 0
                                && brackets.1 == 0
                                && brackets.2 == 0
                                && brackets.3 == 0
                            {
                                break;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
                return Ok(tokens.drain(..i).collect());
            }
            Kinds::Word(txt) => {}
        }
        Err(ParseErr::None)
    }
    fn get_rules() -> Vec<SyntaxNodeHead> {
        vec![
            SyntaxNodeHead {
                kind: Tokens::Keyword(Keywords::If),
                node: vec![
                    NodeOp::Expect(SyntaxNode {
                        kind: Kinds::Value,
                        node: vec![],
                    }),
                    NodeOp::Expect(SyntaxNode {
                        kind: Kinds::Block,
                        node: vec![],
                    }),
                    NodeOp::Maybe(SyntaxNode {
                        kind: Kinds::Word(String::from("else if")),
                        node: vec![
                            NodeOp::Expect(SyntaxNode {
                                kind: Kinds::Value,
                                node: vec![],
                            }),
                            NodeOp::Expect(SyntaxNode {
                                kind: Kinds::Block,
                                node: vec![],
                            }),
                            NodeOp::Jmp(3),
                        ],
                    }),
                    NodeOp::Maybe(SyntaxNode {
                        kind: Kinds::Word(String::from("else")),
                        node: vec![NodeOp::Expect(SyntaxNode {
                            kind: Kinds::Block,
                            node: vec![],
                        })],
                    }),
                ],
            },
            SyntaxNodeHead {
                kind: Tokens::Keyword(Keywords::Switch),
                node: vec![
                    NodeOp::Expect(SyntaxNode {
                        kind: Kinds::Word(String::from("{")),
                        node: vec![
                            NodeOp::Maybe(SyntaxNode {
                                kind: Kinds::Value,
                                node: vec![
                                    NodeOp::Expect(SyntaxNode {
                                        kind: Kinds::Block,
                                        node: vec![],
                                    }),
                                    NodeOp::Jmp(2),
                                ],
                            }),
                            NodeOp::Maybe(SyntaxNode {
                                kind: Kinds::Word(String::from("_")),
                                node: vec![NodeOp::Expect(SyntaxNode {
                                    kind: Kinds::Block,
                                    node: vec![],
                                })],
                            }),
                            NodeOp::Jmp(3),
                        ],
                    }),
                    NodeOp::Expect(SyntaxNode {
                        kind: Kinds::Word(String::from("}")),
                        node: vec![],
                    }),
                ],
            },
        ]
    }
    pub struct SyntaxNode {
        kind: Kinds,
        node: Vec<NodeOp>,
    }
    pub struct SyntaxNodeHead {
        kind: Tokens,
        node: Vec<NodeOp>,
    }
    pub enum NodeOp {
        Expect(SyntaxNode),
        Maybe(SyntaxNode),
        Jmp(i128),
        End(Tokens),
    }
    #[derive(PartialEq, Eq)]
    pub enum Kinds {
        Token(Tokens),
        Block,
        Value,
        Word(String),
    }
    #[derive(Debug)]
    pub enum ParseErr {
        None,
        UnexpectedToken,
        FileEnd,
    }
}
