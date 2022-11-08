use std::{env, fs::File, io::Read};
//mod canvas;
//mod parser;
mod runtime;
use ast_parser::ast_parser::generate_ast;
use runtime::*;
use runtime_types::*;
mod reader;
use reader::reader::*;

use crate::test::test::test_init;
mod ast_parser;
mod lexer;
mod test;
mod token_refactor;
mod writer;

/// commands:
/// - run
/// - build
/// - exe
/// - help
/// - test
fn main() {
    let mut args = env::args();
    let path = match args.nth(0) {
        Some(path) => path,
        None => panic!("Path not specified."),
    };
    let cmd = match args.nth(0) {
        Some(cmd) => cmd,
        None => String::from("None"),
    };

    match cmd.as_str() {
        "exe" => {
            let file = match args.nth(0) {
                Some(file) => file,
                None => panic!("File not specified."),
            };
            let mut ctx = read_file(file, Context::new());
            ctx.run();
        }
        "build" => {
            let file = match args.nth(0) {
                Some(file) => file,
                None => panic!("File not specified."),
            };
            //let file_path = Path::new(&path).join("..").join(&file);
            println!("Compilation for '{file}' starts.");
            let mut string = String::new();
            let mut file =
                File::open(file).expect(&format!("File not found. ({})", path).to_owned());
            file.read_to_string(&mut string).expect("neco se pokazilo");
            use lexer::tokenizer::*;
            /*let idx = find("fun(dvacetz). .nevim nic");
            println!("{:?}", match_keyword(&"fun(dvacetz). .nevim nic"[..idx]))*/
            println!("{:?}", parse(string, true).0);
        }
        "run" => {
            let file = match args.nth(0) {
                Some(file) => file,
                None => panic!("File not specified."),
            };
            //let file_path = Path::new(&path).join("..").join(&file);
            println!("Compilation for '{file}' starts.");
            let mut string = String::new();
            let mut file =
                File::open(file).expect(&format!("File not found. ({})", path).to_owned());
            file.read_to_string(&mut string).expect("neco se pokazilo");
            todo!();
        }
        "test" => {
            use std::time::SystemTime;
            let start_time = SystemTime::now();
            let mut ctx = Context::new();
            test_init(None, &mut ctx);
            match args.nth(0) {
                Some(file) => writer::writer::write(&ctx.code, &ctx.stack, &file),
                None => {
                    println!("file not specified");
                }
            };
            let build_time = SystemTime::now();
            ctx.run();
            let finish_time = SystemTime::now();
            println!("\nProcess ended.");
            println!(
                "Total start time: {} ms",
                build_time.duration_since(start_time).unwrap().as_millis()
            );
            println!(
                "Total run time: {} ms",
                finish_time.duration_since(build_time).unwrap().as_millis()
            );
            if let Types::Usize(num) = ctx.registers[0] {
                if num == 1 {
                    println!("\nYou have triggered post-process data report.");
                    println!("If this is an accident, do not load Usize(1) to register(0) at the end of execution.");
                    println!("Heap: {:?}", ctx.heap);
                    println!("stack: {:?}", ctx.stack);
                    println!("registers: {:?}", ctx.registers);
                    println!("heap_reg: {:?}", ctx.heap_registry);
                }
            }
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
                    println!("{node:?}\n");
                }
            }else{
                println!("failed to parse AST properly")
            }
        }
        _ => {
            println!("Unknown command: {}", cmd);
            println!("Try help.");
        }
    }
}
