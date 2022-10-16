use std::{env, fs::File, io::Read};
//mod canvas;
mod runtime;
mod parser;
use runtime::*;
use runtime_types::*;
mod reader;
use reader::reader::*;
mod lexer;
mod writer;
mod compile_err;
mod token_refactor;

/// commands:
/// - run
/// - build
/// - exe
/// - help
fn main() {
    let mut args = env::args();
    let path = match args.nth(0) {
        Some(path) => path,
        None => panic!("Path not specified."),
    };
    let cmd = args.nth(0).unwrap();

    if cmd == "exe" {
        let file = match args.nth(0) {
            Some(file) => file,
            None => panic!("File not specified."),
        };
        let mut ctx = read_file(file, Context::new());
        ctx.run();
    }

    if cmd == "run" {
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

    if cmd == "build" {
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
}

//let mut ctx = read_file(path, Context::new());
/*let mut ctx = Context::new();
ctx.stack.push(Types::Int(0));
ctx.stack.push(Types::Int(1));
ctx.stack.push(Types::Usize(1));
ctx.code = vec![
    Instructions::Res(2),
    Instructions::Rd(1, 0),
    Instructions::Rd(0, 1),
    Instructions::Add,
    Instructions::Wr(1),
    Instructions::Debug(0),
    Instructions::Rdc(2, 2),
    Instructions::Alc(0, 2),
    Instructions::Repp(0),
    Instructions::Rd(1, 1),
    Instructions::Debug(0),
    Instructions::Wrp(0, 1),
    Instructions::Goto(1),
];*/
//writer::writer::write(&ctx.code, &ctx.stack);
//ctx.run();
/// Memory:
/// -stack
///     array of values
///     includes stack pointer - each item (last stack alloc, last position in bin)
///     stack: [value; MAX]
///     stack_guide: [{end: usize, code_ptr: usize}; 100]
///     allocation rules:
///         [...hardcoded_values(startup), ...written_values(runtime)]
///         res size: [...current, reg(0), ...other]
/// -heap
///     array of values accessed only with a pointer
///     losing pointer also discards its value, hopefuly
///     heap: [value_ptr; MAX]
/// -global
///     array of values, read-only
///     no stack pointer, compiler handles pointer
/// -registers
///     used to store data for performing transformations
///     0: transformation results
///     1: function return values / other data to transform
///     2: uvidim
/// -value
///     Values::<Type>(value: Type): enum
///
///
///
/// Instructions:
/// wr stack_offset       |01| write          | moves value from reg(0) to stack(<stack_offset> + stack_end)
/// rd stack_offset reg   |02| read           | loads value from stack(<stack_offset> + stack_end) to its reg(<reg>)
/// wrr stack_offset size |03| write range    | moves value from reg(0) to all pos on stack(<stack_offset> + stack_end..<stack_offset> + stack_end + <size>)
/// wrp reg1 reg2         |04| write pointer  | moves value from reg(<reg2>) to stack(<reg1>)
/// rdp reg1 reg2         |05| read pointer   | loads value from stack(reg1) to its reg(<reg2>)
/// rdc stack_pos reg     |06| read constant  | loads value from stack(<stack_pos>) to its reg(<reg>)
/// ptr stack_offset      |07| pointer        | stores pointer to stack(stack_end - <stack_offset>) in reg(0)
/// alc reg size          |08| allocate       | reserves <size> on heap and stores location in registers(<reg>)
/// goto pos              |09| go to          | moves code_pointer to <pos>
/// brnc pos1 pos2        |10| branch         | if reg(0), goto <pos1> else goto <pos2>
/// ret                   |11| return         | moves code_pointer to the last position in stack
/// res size              |12| reserve        | reserves <size> on stack and saves current reg(0)
/// mov reg1 reg2         |13| move           | moves value of <reg1> to <reg2>
/// ADD                   |14| add            | reg(0) is set to the result of operation: reg(0) + reg(1)
/// SUB                   |15| subtract       | reg(0) is set to the result of operation: reg(0) - reg(1)
/// MUL                   |16| multiply       | reg(0) is set to the result of operation: reg(0) * reg(1)
/// DIV                   |17| divide         | reg(0) is set to the result of operation: reg(0) / reg(1)
/// MOD                   |18| modulus        | reg(0) is set to the result of operation: reg(0) % reg(1)
/// EQU                   |19| equals         | reg(0) is set to the result of operation: reg(0) = reg(1)
/// GRT                   |20| greater than   | reg(0) is set to the result of operation: reg(0) > reg(1)
/// AND                   |21| and            | reg(0) is set to the result of operation: reg(0) & reg(1)
/// OR                    |22| or             | reg(0) is set to the result of operation: reg(0) | reg(1)
/// NOT                   |23| not            | reg(0) is set to the result of operation: !reg(0)
/// cal procedure args    |24| call           | calls external <procedure>(program state, <args>) written in rust (for syscalls etc..)
/// end                   |25| end            | terminates program
/// dalc reg              |26| de-allocate    | frees heap(<reg>)
/// ralc reg size         |27| allocate resize| resizes heap(<reg>) for <size>; additional space is filled with null
/// idx reg1 reg2         |28| Index          | gets pointer from reg(<reg1>) repairs it and adds reg(<reg2>)
/// repp reg              |29| Repair pointer | Repairs pointer in reg(<reg>)
/// LESS                  |30| less than      | reg(0) is set to the result of operation: reg(0) < reg(1)
/// Gotop reg             |31| goto pointer   | moves code pointer to reg(<reg>)
///
///
///
/// Compilation rules:
/// {
///     let a: int = expression
/// }
/// -notes: positions.new(a, int, 1)
/// -print <expression> as expression
/// wr (positions.get(a))
/// {
///     let b: [int, 10]
/// }
/// -notes: positions.new(b, int, 10)
/// {
///     struct c{
///         a: int,
///         b: char,
///         c: [char, 30],
///     }
///     let d: c = {
///         a: 5,
///         b: 'a',
///         c: ['a', 30]
///     }
/// }
///
///
/// Excercise:
/// fn my_func(danda: int) -> bool{
///     return true;
/// }
/// int danda2 = danda1 + 50;
/// bool danda3 = my_func(danda2);
/// if danda3 {
///     danda2 += 3;
/// } else {
///     danda2 -= danda2;
/// }
///     
/// Translates to:
/// HEAD
/// hc_vars{
///     true: bool,
///     50: int,
///     3: int
/// }
/// CODE
/// #my_func
/// rdc 0 1
/// ret
/// #rest
/// rd (danda1.stack_offset) 0
/// rdc 1 1
/// add 0 1
/// wr (danda2.stack_offset) 0
/// rd (danda2.stack_offset) 0
/// wr (my_func.danda.stack_offset = 0) 0
/// res (sizeof(int))
/// goto (my_func)
/// wr (danda3.stack_offset) 1
/// rd (danda3.stack_offset) 0
/// brnc (pos1) (pos2)
/// #pos1
/// rdc 2 1
/// add
/// goto (if-statement-end)
/// #pos2
/// rd (danda2.stack_offset) 0
/// sub
/// #if-statement-end
/// end
///
/// scan order:
/// write all definitions, global vars, function headers and positions in source
const a: f32 = 1.;
