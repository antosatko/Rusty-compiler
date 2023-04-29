use crate::{intermediate, lexer::tokenizer::{Tokens, self}, ast_parser::ast_parser::generate_ast, tree_walker::tree_walker::{generate_tree, Node}, lexing_preprocessor::lexing_preprocessor};
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