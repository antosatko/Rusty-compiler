pub mod compiler_data {
    /// all of the defined types/variables (enum, struct, function) in the current scope will be registered here
    pub struct Dictionary {
        pub functions: Vec<Function>,
        pub enums: Vec<Enum>,
        pub structs: Vec<Struct>,
        pub variables: Vec<Variable>,
        pub identifiers: Vec<(String, Types)>,
    }
    pub struct Function {
        /// function identifiers will be changed to allow for function overload
        /// name mangler rules: "{identifier}:{args.foreach("{typeof}:")}"
        /// example:
        /// fun myFun(n: int, type: char): int
        /// fun nothing()
        /// translates to:
        /// "myFun:int:char"
        /// "nothing:"
        pub identifier: String,
        /// type of args in order
        pub args: Vec<Types>,
        /// size needed to allocate on stack while function call (args.len() included)
        pub stack_size: Option<usize>,
        /// location in bytecode, so runtime knows where to jump
        pub location: Option<usize>,
        pub return_type: Types,
        /// location in source code
        pub src_loc: usize,
        /// point
        /// Rusty danda specific feature lets you jump to a specific place in a function
        /// fun foo(a:int, b:int) {
        ///     // do something with variable a
        ///     'initialized(b: int);
        ///     // do something with variable b only
        /// }
        /// foo(1, 2); // normal call
        /// foo'initialized(2) // call from point 'initialized
        /// disclaimer: I am fully aware that this feature goes against a lot of good practices.
        /// I just want to offer some flexibility for my language.
        /// identifier, location, source location
        pub points: Vec<(String, usize, usize)>,
    }
    pub struct Enum {
        pub identifier: String,
        /// enum values and their offset
        /// enum ErrCode { Continue = 100, SwitchingProtocols, ..., Ok = 200, ... }
        pub keys: Vec<(String, usize)>,
        /// location in source code
        pub src_loc: usize,
        pub methods: Vec<Function>,
    }
    pub struct Struct {
        pub identifier: String,
        pub keys: Vec<(String, Types)>,
        /// location in source code
        pub src_loc: usize,
        pub methods: Vec<Function>,
    }
    pub struct Variable {
        pub kind: Types,
        pub identifier: String,
    }
    /// identifiers can not contain these characters: + - * / = % ; : , . ({<[]>}) & | ! ? " '
    /// map: let i: Int = 32; i = i + 63;
    ///     - match {keyword? => keyword(?), value? => value(?)} => keyword(let), identifier("i"), match {: => Type, = => None} => Type(Int), operator(=), value(32);
    ///     - match {keyword? => keyword(?), value? => value} => value, value("i"), operator(=), value("i"), operator(+), value(63);
    pub enum Types {
        Int,
        Float,
        Usize,
        Char,
        Byte,
        Bool,
        Null,
        /// refference type
        Pointer(Box<Types>),
        /// type of an array, lenght
        Array(Box<Types>, usize),
        /// non-primmitive types holding their identifiers
        Function(String),
        Enum(String),
        Struct(String),
    }
    impl Dictionary {
        pub fn new() -> Self {
            Dictionary {
                functions: vec![],
                enums: vec![],
                structs: vec![],
                variables: vec![],
                identifiers: vec![],
            }
        }
        fn index_of(&self, identifier: String) -> Option<usize> {
            let mut i = 0;
            loop {
                if i >= self.identifiers.len() {
                    return None;
                }
                if self.identifiers[i].0 == identifier {
                    return Some(i);
                }
                i += 1;
            }
        }
        fn type_of(&self, idx: usize) -> &Types {
            &self.identifiers[idx].1
        }
    }
}




















/*
match &this.nodes[cursor] {
    NodeType::Expect(tok) => {
        if let Tokens::Text(txt) = &tok.kind {
            
        }else {
            if token_cmp(tok.kind.to_owned(), tokens[*idx].to_owned()) {

            }else {
                return Err(AnalyzeErr::Expected(tok.kind.clone(), tokens[*idx].clone()))
            }
        }
    }
    NodeType::Maybe(tok) => {}
    NodeType::ArgsCondition(con) => {}
    NodeType::Command(comm) => {
        if let Tokens::Text(txt) = &comm.kind {
            match txt.as_str() {
                "end" => {
                    return Ok(result) ;
                }
                "err" => {
                    return Err(AnalyzeErr::Placeholder) ;
                }
                _ => {
                    println!("Unrecognized command: {}", &txt);
                }
            }
        }
    }
}
cursor += 1;*/