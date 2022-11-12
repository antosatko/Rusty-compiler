pub mod syntax {
    use crate::lexer::compiler_data::*;
    fn get_rules() -> Vec<SyntaxNodeHead> {
        vec![
            SyntaxNodeHead {
                kind: Keywords::If,
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
                kind: Keywords::Switch,
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
        kind: Keywords,
        node: Vec<NodeOp>,
    }
    pub enum NodeOp {
        Expect(SyntaxNode),
        Maybe(SyntaxNode),
        Jmp(i128),
        End(Tokens),
    }
    pub enum Kinds {
        Token(Tokens),
        Block,
        Value,
        Word(String),
    }
}
