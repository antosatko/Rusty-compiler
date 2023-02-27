pub mod runtime_types {
    /// context for a single thread of execution (may include multiple threads in future updates)
    pub struct Context {
        pub stack: Vec<Types>,
        pub call_stack: [CallStack; 100],
        pub stack_ptr: usize,
        pub registers: [Types; 4],
        pub code: Vec<Instructions>,
        pub code_ptr: usize,
        pub heap: Vec<Types>,
        pub heap_registry: Vec<HeapRegistry>,
    }
    /// a structure used to register data on heap
    #[derive(Clone, Copy, Debug)]
    pub struct HeapRegistry {
        pub prev: Option<usize>,
        pub next: Option<usize>,
        pub len: usize,
        pub dels: usize,
    }
    #[allow(unused)]
    #[derive(Clone, Copy, Debug)]
    pub enum Types {
        Int(i32),
        Float(f64),
        Usize(usize),
        Char(char),
        Byte(u8),
        Bool(bool),
        Pointer(usize, PointerTypes),
        CodePointer(usize),
        Null,
    }
    use std::fmt;
    impl fmt::Display for Types {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if f.alternate() {
                match *self {
                    Types::Bool(_) => write!(f, "Bool"),
                    Types::Byte(_) => write!(f, "Byte"),
                    Types::Char(_) => write!(f, "Char"),
                    Types::CodePointer(_) => write!(f, "CodePointer"),
                    Types::Float(_) => write!(f, "Float"),
                    Types::Int(_) => write!(f, "Int"),
                    Types::Null => write!(f, "Null"),
                    Types::Pointer(_, _) => write!(f, "Pointer"),
                    Types::Usize(_) => write!(f, "Usize"),
                }
            } else if f.sign_plus() {
                match *self {
                    Types::Bool(bol) => {
                        write!(f, "Bool<{bol}>")
                    }
                    Types::Byte(byte) => write!(f, "Byte<{byte}>"),
                    Types::Char(char) => write!(f, "Char<{char}>"),
                    Types::CodePointer(loc) => write!(f, "CodePointer<{loc}>"),
                    Types::Float(num) => write!(f, "Float<{num}>"),
                    Types::Int(num) => write!(f, "Int<{num}>"),
                    Types::Null => write!(f, "Null"),
                    Types::Pointer(loc, kind) => write!(f, "Pointer<{loc}, {kind}>"),
                    Types::Usize(num) => write!(f, "Usize<{num}>"),
                }
            } else {
                match *self {
                    Types::Bool(bol) => {
                        write!(f, "{bol}")
                    }
                    Types::Byte(byte) => write!(f, "{byte}"),
                    Types::Char(char) => write!(f, "{char}"),
                    Types::CodePointer(loc) => write!(f, "{loc}"),
                    Types::Float(num) => write!(f, "{num}"),
                    Types::Int(num) => write!(f, "{num}"),
                    Types::Null => write!(f, "Null"),
                    Types::Pointer(loc, _) => write!(f, "{loc}"),
                    Types::Usize(num) => write!(f, "{num}"),
                }
            }
        }
    }
    /// runtime
    #[derive(Clone, Copy, Debug)]
    pub enum PointerTypes {
        /// location on stack
        ///
        /// never expires
        Stack,
        /// heap pointer in "broken state"
        /// needs to be transformed into heap pointer
        ///
        /// never expires
        HeapReg,
        /// location on heap
        ///
        /// may expire at any time
        Heap,
    }
    impl fmt::Display for PointerTypes {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                PointerTypes::Heap => write!(f, "Heap"),
                PointerTypes::HeapReg => write!(f, "HeapRaw"),
                PointerTypes::Stack => write!(f, "Stack"),
            }
        }
    }
    /// complete list of runtime instructions
    #[allow(unused)]
    #[derive(Clone, Copy, Debug)]
    pub enum Instructions {
        /// debug reg        | prints value of reg(<reg>)
        Debug(usize),
        /// write stack_offset           | moves value from reg(0) to stack(stack_end - <stack_offset>)
        Wr(usize),
        /// read stack_offset reg | reads value from stack(stack_end - <stack_offset>) to its reg(<reg>)
        Rd(usize, usize),
        /// write pointer value_reg   | moves value from reg(<reg2>) to stack(<reg1>)
        Wrp(usize, usize),
        /// read_pointer pointer_reg reg | reads value from stack(reg1) to its reg(<reg2>)
        Rdp(usize, usize),
        /// read constant    | reads value from stack(<stack_pos>) to its reg(<reg>)
        Rdc(usize, usize),
        /// pointer stack_pos | stores pointer to stack(stack_end - <stack_offset>) in reg(0)
        Ptr(usize),
        /// Index pointer idx<usize> | gets pointer from reg(<reg1>) repairs it and adds reg(<reg2>)
        Idx(usize, usize),
        /// Repair pointer   | Repairs pointer in reg(<reg>)
        Repp(usize),
        /// allocate pointer size_reg | reserves <size> on heap and stores location in registers(<reg>)
        Alc(usize, usize),
        /// deallocate pointer | frees heap(<reg>)
        Dalc(usize),
        /// reallocate pointer size_reg | resizes heap(<reg>) for <size>; additional space is filled with null
        RAlc(usize, usize),
        /// goto pos         | moves code_pointer to <pos>
        Goto(usize),
        /// goto pos_reg     | moves code pointer to reg(<reg>)
        Gotop(usize),
        /// branch pos1 pos2 | if reg(0), goto <pos1> else goto <pos2>
        Brnc(usize, usize),
        /// return           | moves code_pointer to the last position in stack retrieved from stack
        Ret,
        /// register return  | returns registers to their freezed previous state
        RRet,
        /// reserve size     | reserves <size> on stack and saves current reg(0)
        Res(usize),
        /// move reg1 reg2   | moves value of <reg1> to <reg2>
        Mov(usize, usize),
        /// add              | reg(0) is set to the result of operation: reg(0) + reg(1)
        Add,
        /// subtract         | reg(0) is set to the result of operation: reg(0) - reg(1)
        Sub,
        /// multiply         | reg(0) is set to the result of operation: reg(0) * reg(1)
        Mul,
        /// divide           | reg(0) is set to the result of operation: reg(0) / reg(1)
        Div,
        /// modulus          | reg(0) is set to the result of operation: reg(0) % reg(1)
        Mod,
        /// equals           | reg(0) is set to the result of operation: reg(0) = reg(1)
        Equ,
        /// greater than     | reg(0) is set to the result of operation: reg(0) > reg(1)
        Grt,
        /// less than        | reg(0) is set to the result of operation: reg(0) < reg(1)
        Less,
        /// and              | reg(0) is set to the result of operation: reg(0) & reg(1)
        And,
        /// or               | reg(0) is set to the result of operation: reg(0) | reg(1)
        Or,
        /// not              | reg(0) is set to the result of operation: !reg(0)
        Not,
        /// call             | calls external <procedure>(program state, <args>) written in rust (for syscalls etc..)
        Cal(usize, usize),
        /// end              | terminates program
        End,
        //TODO: add to compiler
        /// cast reg1 reg2   | casts value of reg1 to the type of reg2 and stores in reg1
        Cast(usize, usize),
        /// length reg       | sets reg to Usize(size of heap object)
        Len(usize),
        /// type val type    | sets reg(type) to bool(typeof(val) == typeof(type))
        Type(usize, usize),
    }
    impl fmt::Display for Instructions {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let str = match *self {
                Instructions::Add => "Addition",
                Instructions::Alc(_, _) => "Allocation",
                Instructions::And => "And",
                Instructions::Brnc(_, _) => "Branch",
                Instructions::Cal(_, _) => "Call",
                Instructions::Dalc(_) => "Deallocation",
                Instructions::Debug(_) => "Debug",
                Instructions::Div => "Division",
                Instructions::End => "End",
                Instructions::Equ => "Equality",
                Instructions::Goto(_) => "GoTo",
                Instructions::Gotop(_) => "GoToDyn",
                Instructions::Grt => "Greater",
                Instructions::Idx(_, _) => "Indexing",
                Instructions::Less => "Lesser",
                Instructions::Mod => "Modulus",
                Instructions::Mov(_, _) => "Move",
                Instructions::Mul => "Multiplication",
                Instructions::Not => "Not",
                Instructions::Or => "Or",
                Instructions::Ptr(_) => "StackPointer",
                Instructions::RAlc(_, _) => "Reallocation",
                Instructions::RRet => "RegisterReturn",
                Instructions::Rd(_, _) => "Read",
                Instructions::Rdc(_, _) => "ReadConst",
                Instructions::Rdp(_, _) => "Dereference",
                Instructions::Repp(_) => "ReapirPointer",
                Instructions::Res(_) => "Reserve",
                Instructions::Ret => "Return",
                Instructions::Sub => "Subtract",
                Instructions::Wr(_) => "Write",
                Instructions::Wrp(_, _) => "WriteRef",
                Instructions::Cast(_, _) => "Casting",
                Instructions::Len(_) => "Length",
                Instructions::Type(_, _) => "TypeOf",
            };
            write!(f, "{str}")
        }
    }
    /// holds information of where to jump after function call ends
    #[derive(Clone, Copy)]
    pub struct CallStack {
        pub reg_freeze: [Types; 4],
        pub end: usize,
        pub code_ptr: usize,
    }
}
