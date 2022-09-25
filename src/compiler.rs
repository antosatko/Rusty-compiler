pub mod compiler {
    use super::compiler_data::*;
    const RESERVED_CHARS: &str = "+-*/=%;:,.({<[]>})&|!?\"'\\";
    pub const KEYWORDS: [&str; 18] = [
        "if", "switch", "let", "const", "fun", "struct", "enum", "loop", "for", "while", "do",
        "return", "break", "continue", "lib", "use", "//", "/*",
    ];
    pub fn compile_file(file: String, target: String) {
        let mut global = Dictionary::new();
        // let mut tree: Vec = vec![];
    }
    pub fn validate_identifier(string: &str) -> bool {
        if find_ws_rc(string) == string.len() {
            if match_keyword(string) == Keywords::Value {
                return true;
            }
            // syntax err: keyword names are reserved
            return false;
        }
        // syntax err: identifier contains reserved character
        false
    }
    /// returns index of found keyword in const KEYWORDS
    pub fn find_keyword(string: &mut str) -> Option<usize> {
        for (i, key) in KEYWORDS.iter().enumerate() {
            if let Some(idx) = string.trim().find(key) {
                if idx == 0 {
                    return Some(i);
                }
            }
        }
        None
    }
    pub fn match_keyword(string: &str) -> Keywords {
        match string {
            "if" => Keywords::If,
            "switch" => Keywords::Switch,
            "let" => Keywords::Let,
            "const" => Keywords::Const,
            "fun" => Keywords::Function,
            "struct" => Keywords::Struct,
            "enum" => Keywords::Enum,
            "loop" => Keywords::Loop,
            "for" => Keywords::For,
            "while" => Keywords::While,
            "do" => Keywords::DoWhile,
            "return" => Keywords::Return,
            "break" => Keywords::Break,
            "continue" => Keywords::Continue,
            "lib" => Keywords::Lib,
            "use" => Keywords::Use,
            "//" => Keywords::CommentLine,
            "/*" => Keywords::CommentBlock,
            _ => Keywords::Value,
        }
    }
    pub fn global_keyword(keyword: Keywords) -> bool {
        match keyword {
            Keywords::Lib => true,
            Keywords::Use => true,
            Keywords::Const => true,
            Keywords::Function => true,
            Keywords::Struct => true,
            Keywords::Enum => true,
            Keywords::CommentLine => true,
            Keywords::CommentBlock => true,
            _ => false,
        }
    }
    pub fn find_ws_rc(expression: &str) -> usize {
        fn compare(original: &mut usize, compared: Option<usize>) {
            if let Some(compared) = compared {
                if compared < *original {
                    *original = compared
                }
            }
        }
        let idx = {
            let mut lowest_idx = expression.len();
            for _char in RESERVED_CHARS.chars() {
                compare(&mut lowest_idx, expression.find(_char));
            }
            compare(&mut lowest_idx, expression.find(char::is_whitespace));
            lowest_idx
        };
        idx
    }
}

mod compiler_data {
    /// all of the defined types/variables (enum, struct, function) in the current scope will be registered here
    pub struct Dictionary {
        pub functions: Vec<Function>,
        pub enums: Vec<Enum>,
        pub structs: Vec<Struct>,
        pub variables: Vec<(String, Types, Identyfiers)>,
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
        /// identifier and type of arguments
        pub args: Vec<(String, Types)>,
        /// size needed to allocate on stack while function call (args.len() included)
        pub stack_size: usize,
        /// location in code, so runtime knows where to jump
        pub location: usize,
        pub return_type: Types,
    }
    pub struct Enum {
        pub identifier: String,
        /// enum values and their offset
        /// enum ErrCode { Continue = 100, SwitchingProtocols, ..., Ok = 200, ... }
        pub keys: Vec<(String, usize)>,
    }
    pub struct Struct {
        pub identifier: String,
        pub keys: Vec<(String, Types)>,
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
    /// "+-*/=%;:,.({<[]>})&|!?\"'\\"
    enum Tokens {
        /// opening 0, closing 1
        Parenteses(bool),
        /// opening 0, closing 1
        CurlyBracket(bool),
        /// opening 0, closing 1
        SquareBracket(bool),
        /// opening 0, closing 1
        AngleBracket(bool),
        Operator(Operators),
        Colon,
        Dot,
        Semicolon,
        Comma,
        /// content
        String(String),
        Char(char),
        Number(u128),
        Float(f64),
        /// variable name
        Variable(String),
        /// function name, args
        Call(String, Vec<String>),
    }
    enum Operators {
        Add,
        Sub,
        Mul,
        Div,
        Mod,
        AddEq,
        SubEq,
        MulEq,
        DivEq,
        Compare,
        Equal,
        NotEqual,
        And,
        Or,
        Not,
    }
    /// compiler iterates over source, looking for theese keywords
    /// dependent keywords:
    ///     Break
    ///     Continue
    /// global allowed:
    ///     Lib
    ///     Use
    ///     Conts
    ///     Fun
    ///     Struct
    ///     Enum
    ///     CommentLine
    ///     CommentBlock
    #[derive(Debug, PartialEq)]
    pub enum Keywords {
        /// "if"
        /// statement
        /// code_block
        /// "else if" ?
        ///     statement
        ///     code_block
        ///     <<<
        /// "else" ?
        ///     code_block
        If,
        /// "switch"
        /// value
        /// {
        /// comparing || "_" ?
        ///     code_block
        ///     <<
        /// }
        Switch,

        /// "let"
        /// identifier
        ///     : ?
        ///         type
        /// = ?
        ///     value
        /// , ?
        ///     <<<<<<
        /// ;  
        Let,
        /// "const"
        /// identifier
        ///     : ?
        ///         type
        /// = ?
        ///     value
        /// , ?
        ///     <<<<<<
        /// ;  
        Const,
        /// "fun"
        /// identifier
        /// (
        ///     value ?
        ///         , ?
        ///             <<
        /// )
        /// : ?
        ///     return_type
        /// code_block
        Function,
        /// "struct"
        /// identifier
        /// {
        /// key ?
        ///     :
        ///     type
        ///     , ?
        ///         <<<<
        /// }
        Struct,
        /// "enum"
        /// identifier
        /// {
        /// identifier ?
        ///     = ?
        ///         number
        ///         , ?
        ///             <<<<
        ///     , ?
        ///         <<<
        /// }
        Enum,

        /// "loop"
        /// code_block {breakIf: <Continue>, <Break>}
        Loop,
        /// "for"
        /// jeste uvidim
        /// code_block {breakIf: <Continue>, <Break>}
        For,
        /// "while"
        /// statement
        /// code_block {breakIf: <Continue>, <Break>}
        While,
        /// "do"
        /// code_block
        /// statement {breakIf: <Continue>, <Break>}
        /// ;
        DoWhile,

        /// "return"
        /// value ?
        /// ;
        Return,
        /// "break"
        /// value ?
        /// ;
        Break,
        /// "continue"
        Continue,

        /// "lib"
        /// file_identifier
        /// "as" ?
        ///     alias
        /// ;
        Lib,
        /// "use"
        /// file_identifier
        /// "as" ?
        ///     alias
        /// ;
        Use,

        /// checked after all other keywords
        /// value
        /// ;
        Value,

        /// "//"
        /// _
        /// "\n"
        CommentLine,
        /// "/*"
        /// _
        /// "*/"
        CommentBlock,
    }
    pub enum Identyfiers {
        Const,
        Imutable,
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
