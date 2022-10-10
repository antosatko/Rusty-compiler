pub mod compiler {
    use std::collections::HashMap;

    use super::compiler_data::*;
    use crate::{syntax::syntax::*, token_refactor::refactorer::refactor};
    const RESERVED_CHARS: &str = " +-*/=%;:,.({<[]>})&|!?\"'\\";
    pub const KEYWORDS: [&str; 19] = [
        "if", "switch", "let", "const", "fun", "struct", "trait", "enum", "loop", "for", "while", "do",
        "yeet", "break", "continue", "lib", "use", "//", "/*",
    ];
    pub const GLOBALS: [Keywords; 9] = [
        Keywords::Lib,
        Keywords::Use,
        Keywords::Const,
        Keywords::Function,
        Keywords::Struct,
        Keywords::Trait,
        Keywords::Enum,
        Keywords::CommentLine,
        Keywords::CommentBlock,
    ];
    pub const VALUE_HOLDERS: [Keywords; 3] = [
        Keywords::If,
        Keywords::Loop,
        Keywords::Switch,
    ];
    pub fn parse(file: String, target: String) {
        let mut tokens: Vec<Tokens> = vec![];

        let mut i = 0;
        while i < file.len() {
            let res = get_token(&file[i..]);
            if i % 100 < 6 {
                println!("{i}");
            }
            tokens.push(res.0);
            i += res.1;
        }
        println!("Total len: {}", tokens.len());
        if let Ok(refactored) = refactor(tokens) {
            tokens = refactored;
            /*for token in &tokens {
                println!("{:?}", token);
            }*/
            println!("Total len: {}", tokens.len());
        }else {
            println!("neco se pokazilo")
        }
    }
    pub fn get_token(line: &str) -> (Tokens, usize) {
        if let Some(idx) = find_keyword(&line) {
            // keyword
            (Tokens::Keyword(parse_keyword(&KEYWORDS[idx])), KEYWORDS[idx].len())
        }else {
            // expression
            let len = find_ws_str(line, &RESERVED_CHARS);
            let len = if len == 0 {1} else {len};
            let str = &line[0..len];
            let token = parse_token(&str);
            (token, str.len())
        }
    }
    /// returns index of found keyword in const KEYWORDS
    pub fn find_keyword(string: &str) -> Option<usize> {
        for (i, key) in KEYWORDS.iter().enumerate() {
            if let Some(idx) = string.find(key) {
                if idx == 0 {
                    return Some(i);
                }
            }
        }
        None
    }
    pub fn parse_keyword(string: &str) -> Keywords {
        match string {
            "if"        => Keywords::If,
            "switch"    => Keywords::Switch,
            "let"       => Keywords::Let,
            "const"     => Keywords::Const,
            "fun"       => Keywords::Function,
            "struct"    => Keywords::Struct,
            "trait"     => Keywords::Trait,
            "enum"      => Keywords::Enum,
            "loop"      => Keywords::Loop,
            "for"       => Keywords::For,
            "while"     => Keywords::While,
            "do"        => Keywords::DoWhile,
            "yeet"      => Keywords::Return,
            "break"     => Keywords::Break,
            "continue"  => Keywords::Continue,
            "lib"       => Keywords::Lib,
            "use"       => Keywords::Use,
            "//"        => Keywords::CommentLine,
            "/*"        => Keywords::CommentBlock,
            _           => Keywords::Value,
        }
    }
    pub fn parse_token(string: &str) -> Tokens {
        // +-*/=%;:,.({<[]>})&|!?"'\
        match string {
            "+"     => Tokens::Operator(Operators::Add),
            "-"     => Tokens::Operator(Operators::Sub),
            "*"     => Tokens::Operator(Operators::Mul),
            "/"     => Tokens::Operator(Operators::Div),
            "="     => Tokens::Operator(Operators::Equal),
            "%"     => Tokens::Operator(Operators::Mod),
            "&"     => Tokens::Operator(Operators::And),
            "|"     => Tokens::Operator(Operators::Or),
            "!"     => Tokens::Operator(Operators::Not),
            "?"     => Tokens::Optional,
            ";"     => Tokens::Semicolon,
            ":"     => Tokens::Colon,
            ","     => Tokens::Comma,
            "."     => Tokens::Dot,
            "\""    => Tokens::DoubleQuotes,
            r"'"    => Tokens::Quotes,
            "("     => Tokens::Parenteses(false),
            ")"     => Tokens::Parenteses(true),
            "{"     => Tokens::CurlyBracket(false),
            "}"     => Tokens::CurlyBracket(true),
            "<"     => Tokens::AngleBracket(false),
            ">"     => Tokens::AngleBracket(true),
            "["     => Tokens::SquareBracket(false),
            "]"     => Tokens::SquareBracket(true),
            " "     => Tokens::Space,
            _       => Tokens::Text(string.to_string())
        }
    }
    fn compare(original: &mut usize, compared: Option<usize>) {
        if let Some(compared) = compared {
            if compared < *original {
                *original = compared
            }
        }
    }
    pub fn find_ws_str(expression: &str, str: &str) -> usize {
        let idx = {
            let mut lowest_idx = expression.len();
            for char in str.chars() {
                compare(&mut lowest_idx, expression.find(char));
            }
            compare(&mut lowest_idx, expression.find(char::is_whitespace));
            lowest_idx
        };
        idx
    }
}

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
    /// "+-*/=%;:,.({<[]>})&|!?\"'\\"
    #[derive(Debug, PartialEq, Clone)]
    pub enum Tokens {
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
        Quotes,
        DoubleQuotes,
        Optional,
        Space,
        Block,
        /// content
        String(String),
        Char(char),
        Number(u128),
        Float(f64),
        /// variable name
        Identifier(String),
        Keyword(Keywords),
        None,
        /// in case we can not identify token at the moment
        Text(String),
        DoubleColon,
    }
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Operators {
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
        DoubleEq,
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
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Keywords {
        /// value
        /// code_block
        /// "else if" ?
        ///     value
        ///     code_block
        ///     <<<
        /// "else" ?
        ///     code_block
        If,
        /// value
        /// {
        /// value || "_" ?
        ///     code_block
        ///     <<
        /// }
        Switch,

        /// identifier
        ///     : ?
        ///         type
        /// = ?
        ///     value
        /// , ?
        ///     <<<<<<
        /// ;  
        Let,
        /// identifier
        ///     : ?
        ///         type
        /// = ?
        ///     value
        /// , ?
        ///     <<<<<<
        /// ;  
        Const,
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
        /// identifier
        /// {
        /// Trait_identifier ?
        ///     <
        /// key ?
        ///     :
        ///     type
        ///     , ?
        ///         <<<<<
        /// }
        Struct,
        /// identifier
        /// {
        /// Function ?
        ///     <
        /// key ?
        ///     :
        ///     type
        ///     , ?
        ///         <<<<<
        /// }
        Trait,
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

        /// code_block {breakIf: [<Continue>, <Break>]}
        Loop,
        /// jeste uvidim
        /// code_block {breakIf: [<Continue>, <Break>]}
        For,
        /// value
        /// code_block {breakIf: [<Continue>, <Break>]}
        While,
        /// code_block
        /// value {breakIf: [<Continue>, <Break>]}
        /// ;
        DoWhile,

        /// value ?
        /// ;
        Return,
        /// value ?
        /// ;
        Break,
        Continue,

        /// file_identifier
        /// "as" ?
        ///     alias
        /// ;
        Lib,
        /// file_identifier
        /// "as" ?
        ///     alias
        /// ;
        Use,

        /// checked after all other keywords
        /// value
        /// ;
        Value,

        /// _
        /// "\n"
        CommentLine,
        /// _
        /// "*/"
        CommentBlock,
    }
    pub enum Modifyers {
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
