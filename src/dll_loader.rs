use crate::{intermediate::{self, AnalyzationError::ErrType}, lexer::tokenizer::{Tokens, self}, ast_parser::ast_parser::generate_ast, tree_walker::tree_walker::{generate_tree, Node}, lexing_preprocessor::lexing_preprocessor, expression_parser::ValueType};
use intermediate::dictionary::*;
use lexing_preprocessor::*;


pub fn load(string: &mut Vec<u8>) -> Result<Dictionary, String> {
    let (mut tokens, mut lines, mut errs) = tokenizer::tokenize(string, true);
    let ast = if let Some(ast) = generate_ast("ast/registry.ast") {
        ast
    } else {
        return Err("".to_owned());
    };
    let tree = if let Some(tree) = generate_tree(&tokens, &ast, &lines) {
        println!("{:?}", tree);
        tree
    } else {
        return Err("".to_owned());
    };
    let mut dictionary = Dictionary::new();
    for node in step_inside_arr(&tree, "nodes") {
        if let Tokens::Text(name) = &node.name {
            match name.as_str() {
                "KWStruct" => {
                    
                }
                "KWConst" => {
                    println!("current version does not support constants");
                }
                _ => {}
            }   
        } else {
            return Err("".to_owned());
        }
    }
    if errs.len() > 0 {
        for err in errs {
            println!("{:?}", err);
        }
        return Err("".to_owned());
    }

    
    Err("".to_owned())
}

fn from_tree(node: &Node) -> Result<Dictionary, String> {
    let mut dict = Dictionary::new();
    let nodes = step_inside_arr(&node, "nodes");
    Ok(dict)
}

pub struct Dictionary {
    pub functions: Vec<Function>,
    pub structs: Vec<Struct>,
    pub enums: Vec<Enum>,
    pub types: Vec<Type>,
    pub consts: Vec<Const>,
    pub traits: Vec<Trait>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            types: Vec::new(),
            consts: Vec::new(),
            traits: Vec::new(),
        }
    }
}

pub struct Function {
    pub name: String,
    pub args: Vec<(String, ShallowType)>,
    pub return_type: ShallowType,
    pub errorable: bool,
    pub assign: usize,
}

pub struct Struct {
    pub name: String,
    pub fields: Vec<(String, ShallowType)>,
    pub assign: usize,
}

pub struct Enum {
    pub name: String,
    pub variants: Vec<(String, ShallowType, usize)>,
}

pub struct Type {
    pub name: String,
    pub kind: ShallowType,
}

pub struct Const {
    pub name: String,
    pub kind: ShallowType,
    pub value: ValueType,
}

pub struct Trait {
    pub name: String,
    pub functions: Vec<Function>,
    pub assign: usize,
}