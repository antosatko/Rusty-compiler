use std::{env, fs::File, io::Read};
//mod canvas;
mod parser;
mod runtime;
use runtime::*;
use runtime_types::*;
mod reader;
use reader::reader::*;
mod compile_err;
mod lexer;
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
    let cmd = args.nth(0).unwrap();

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
            use lexer::compiler::*;
            /*let idx = find("fun(dvacetz). .nevim nic");
            println!("{:?}", match_keyword(&"fun(dvacetz). .nevim nic"[..idx]))*/
            parse(string, String::new())
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
            println!("Running '{}'.", todo!());
            todo!();
        }
        "test" => {
            use std::thread::sleep;
            use std::time::{Duration, SystemTime};
            let start_time = SystemTime::now();
            let mut ctx = Context::new();
            use Instructions::*;
            use Types::*;
            ctx.stack = vec![
                // initialization of values on stack
                Int(100),
                Int(100),
                Bool(true),
                Int(50000),
                Int(1),
            ];
            // for loop written in dasm
            ctx.code = vec![
                // reserves memory on stack and initializes values
                Res(5),
                // reads values from stack
                Rd(4, 0),
                Rd(0, 1),
                // writes result of their addition
                Add,
                Wr(4),
                // repeats if number is less than stack(1)
                Rd(1, 1),
                Less,
                Brnc(1, 8),
                // prints end result
                Rd(4, 0),
                Debug(0),
                End,
            ];
            match args.nth(0) {
                Some(file) => writer::writer::write(&ctx.code, &ctx.stack, &file),
                None => {
                    println!("file not specified");
                    println!("program will be executed.");
                }
            };
            let build_time = SystemTime::now();
            ctx.run();
            let finish_time = SystemTime::now();
            println!("Process ended.");
            println!("Total build time: {} ms", build_time.duration_since(start_time).unwrap().as_millis());
            println!("Total run time: {} ms", finish_time.duration_since(build_time).unwrap().as_millis());
        }
        _ => {
            println!("Unknown command: {}", cmd);
            println!("Try help.");
        }
    }
}