pub mod syntax {
    use std::collections::HashMap;

    use crate::lexer::compiler_data::*;
    pub fn tokenize(file: String) {
        let _if = SyntaxNode {
            kind: Kinds::Token(Tokens::Keyword(Keywords::If)),
            node: vec![
                NodeOptional::Expect(SyntaxNode {
                    kind: Kinds::Value,
                    node: vec![],
                }),
                NodeOptional::Expect(SyntaxNode {
                    kind: Kinds::Block,
                    node: vec![],
                }),
                NodeOptional::Maybe(SyntaxNode {
                    kind: Kinds::Word(String::from("else if")),
                    node: vec![
                        NodeOptional::Expect(SyntaxNode {
                            kind: Kinds::Value,
                            node: vec![],
                        }),
                        NodeOptional::Expect(SyntaxNode {
                            kind: Kinds::Block,
                            node: vec![],
                        }),
                        NodeOptional::Jmp(3),
                    ],
                }),
                NodeOptional::Maybe(SyntaxNode {
                    kind: Kinds::Word(String::from("else")),
                    node: vec![NodeOptional::Expect(SyntaxNode {
                        kind: Kinds::Block,
                        node: vec![],
                    })],
                }),
            ],
        };
    }
    pub struct SyntaxNode {
        kind: Kinds,
        node: Vec<NodeOptional>,
    }
    pub enum NodeOptional {
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
