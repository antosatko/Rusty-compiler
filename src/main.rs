use ast_parser::ast_parser::generate_ast;
use std::{env, fs::File, hint::black_box, io::Read, time::SystemTime};
use tree_walker::tree_walker::generate_tree;

mod ast_analyzer;
mod ast_parser;
mod lexer;
mod reader;
mod runtime;
mod test;
mod lexing_preprocessor;
mod tree_walker;
mod writer;

fn main() {
    let mut args = env::args();
    let path = match args.nth(0) {
        Some(path) => path,
        None => panic!("Path not specified."),
    };
    let cmd = match args.nth(0) {
        Some(cmd) => cmd,
        None => String::from(""),
    };

    match cmd.as_str() {
        "build" => {
            let file = match args.nth(0) {
                Some(file) => file,
                None => panic!("File not specified."),
            };
            println!("Compilation for '{file}' starts.");
            let mut string = String::new();
            let mut file =
                File::open(file).expect(&format!("File not found. ({})", path).to_owned());
            file.read_to_string(&mut string).expect("neco se pokazilo");
            let string = string.into_bytes();
            use lexer::tokenizer::*;
            let ast = if let Some(ast) = generate_ast("ast/rd.ast") {
                ast
            } else {
                panic!();
            };
            println!("AST loaded.");
            let time = SystemTime::now();
            let mut tokens = tokenize(&string, false);
            tokens.0 = if let Ok(toks) =
                lexing_preprocessor::lexing_preprocessor::refactor(tokens.0, tokens.1, &mut tokens.2)
            {
                tokens.1 = toks.1;
                toks.0
            } else {
                panic!("hruzostrasna pohroma");
            }; //tokenize(&string, true);
            let parsed_tree = generate_tree(&tokens.0, &ast, &tokens.1);
            println!("Parsed.");
            println!(
                "time: {}",
                SystemTime::now().duration_since(time).unwrap().as_millis()
            );
            if true {
                if let Some(nodes) = &parsed_tree {
                    use tree_walker::tree_walker::ArgNodeType;
                    for nod in &nodes.nodes {
                        println!("{:?}", nod.0);
                        match nod.1 {
                            ArgNodeType::Array(arr) => {
                                for arg in arr {
                                    println!("{arg:?}");
                                }
                            }
                            ArgNodeType::Value(val) => {
                                println!("{val:?}");
                            }
                        }
                    }
                }
            }
            black_box(parsed_tree);
        }
        "tokenize" => {
            let file = match args.nth(0) {
                Some(file) => file,
                None => panic!("File not specified."),
            };
            println!("Compilation for '{file}' starts.");
            let mut string = String::new();
            let mut file =
                File::open(file).expect(&format!("File not found. ({})", path).to_owned());
            file.read_to_string(&mut string).expect("neco se pokazilo");
            let string = string.into_bytes();
            use lexer::tokenizer::*;
            let tokens = tokenize(&string, true);
            println!("{:?}", tokens.0);
        }
        "astTest" => {
            let mut file_name = String::from("ast/");
            match args.nth(0) {
                Some(file) => file_name.push_str(&file),
                None => {
                    println!("file not specified");
                    return;
                }
            };
            if let Some(ast) = generate_ast(&file_name) {
                for node in ast {
                    println!("{:?}\n", node)
                }
            } else {
                println!("failed to parse AST properly")
            }
        }
        _ => {
            println!("Unknown command: {}", cmd);
            println!("Try help.");
        }
    }
}
