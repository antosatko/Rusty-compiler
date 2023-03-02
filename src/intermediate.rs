pub mod intermediate {
    use crate::{
        lexer::tokenizer::Tokens,
        tree_walker::tree_walker::{self, ArgNodeType, Node},
    };
    use core::panic;
    use std::collections::HashMap;

    use super::AnalyzationError::{self, ErrType};

    pub fn from_ast(ast: &HashMap<String, tree_walker::ArgNodeType>) {
        let mut global_dict = Dictionary::new();
        let mut errors = Vec::new();
        if let Some(ArgNodeType::Array(entry)) = ast.get("nodes") {
            for node in entry {
                parse_node(&node, &mut global_dict, None, &mut errors);
            }
        }
        println!("global: {global_dict:?}");
        println!("errors: {errors:?}");
    }
    fn parse_node(
        node: &Node,
        global: &mut Dictionary,
        local: Option<&mut Dictionary>,
        errors: &mut Vec<ErrType>,
    ) {
        let name = if let Tokens::Text(name) = &node.name {
            name
        } else {
            panic!()
        };
        match name.as_str() {
            "KWEnum" => {
                let mut result = Enum {
                    identifier: get_ident(&node),
                    keys: vec![],
                    src_loc: 0,
                    methods: vec![],
                };
                for enum_value in step_inside_arr(&node, "values") {
                    let n = if let Tokens::Number(n, _, _) = get_token(&enum_value, "default") {
                        *n
                    } else {
                        let len = result.keys.len() - 1;
                        result.keys[len].1 + 1
                    };
                    let ident = get_ident(&enum_value);
                    for variant in &result.keys {
                        if variant.1 == n {
                            errors.push(ErrType::EnumVariantAssignedNumber(n, (0, 0)))
                        }
                        if variant.0 == ident {
                            errors
                                .push(ErrType::EnumVariantAssignedIdent(ident.to_string(), (0, 0)))
                        }
                    }
                    result.keys.push((ident, n));
                }
                if global.register_id(result.identifier.to_string(), IdentifierKinds::Enum) {
                    global.enums.push(result);
                }else{
                    errors.push(ErrType::ConflictingNames(result.identifier.to_string()))
                }
            }
            "KWType" => {
                let name = get_ident(&node);
                if global.register_id(name.to_string(), IdentifierKinds::Type) {
                    global.types.push(TypeDef {
                        kind: get_type(step_inside_val(&node, "type")),
                        identifier: name,
                        generics: get_generics_decl(&node),
                    })
                }else{
                    errors.push(ErrType::ConflictingNames(name.to_string()))
                }
            },
            "KWStruct" => {
                let mut result = Struct {
                    identifier: get_ident(node),
                    keys: Vec::new(),
                    src_loc: 0,
                    methods: Vec::new(),
                    generics: get_generics_decl(node),
                };
                for key in step_inside_arr(node, "keys") {
                    result
                        .keys
                        .push((get_ident(key), get_type(step_inside_val(key, "type"))))
                }
                if global.register_id(result.identifier.to_string(), IdentifierKinds::Struct) {
                    global.structs.push(result);
                }else{
                    errors.push(ErrType::ConflictingNames(result.identifier.to_string()))
                }
            }
            "KWImport" => {
                let path = if let Tokens::String(path) = get_token(node, "path") {
                    path
                }else {
                    panic!("nemozne")
                };
                let name = if let Some(txt) = try_get_ident(node) {
                    Some(txt)
                }else {
                    None
                };
                match name {
                    Some(name) => {
                        if global.register_id(name.to_string(), IdentifierKinds::Namespace) {
                            // TODO: read file and compile it into dictionary
                        }else{
                            errors.push(ErrType::ConflictingNames(name.to_string()))
                        }
                    }
                    None => {
                        // TODO: read file and compile it into dictionary
                    }
                }
            }
            _ => {}
        }
    }
    fn get_ident(node: &Node) -> String {
        if let Tokens::Text(txt) =
            &step_inside_val(&step_inside_val(&node, "identifier"), "identifier").name
        {
            return txt.to_string();
        }
        panic!();
    }
    fn try_get_ident(node: &Node) -> Option<String> {
        if let Some(val) = try_step_inside_val(&step_inside_val(&node, "identifier"), "identifier") {
            if let Tokens::Text(txt) = &val.name {
                return Some(txt.to_string())
            }
        }
        None
    }
    fn get_type(node: &Node) -> ShallowType {
        let mut refs = 0;
        if let Some(arr) = try_step_inside_arr(&step_inside_val(&node, "ref"), "refs") {
            for ref_type in arr {
                if let Tokens::Ampersant = ref_type.name {
                    refs += 1;
                }
                if let Tokens::Operator(crate::lexer::tokenizer::Operators::And) = ref_type.name {
                    refs += 2;
                }
            }
        }
        let type_ident = step_inside_arr(step_inside_val(&node, "main"), "nodes");
        let mut main = Vec::new();
        for path_part in type_ident {
            if let Tokens::Text(txt) = get_token(path_part, "identifier") {
                main.push(txt.to_string())
            }
        }
        ShallowType {
            refs,
            main,
            generics: get_generics_expr(node),
        }
    }
    fn get_generics_expr(node: &Node) -> GenericExpr {
        let mut result = Vec::new();
        if let Some(arr) = try_step_inside_arr(step_inside_val(node, "generic"), "types") {
            for generic_expr in arr {
                result.push(get_type(generic_expr));
            }
        }
        result
    }
    fn get_generics_decl<'a>(node: &'a Node) -> Vec<GenericDecl> {
        let mut generics = Vec::new();
        if let Some(arr) = try_step_inside_arr(step_inside_val(&node, "generic"), "identifiers") {
            for generic in arr {
                let mut traits = Vec::new();
                for ident in step_inside_arr(generic, "traits") {
                    if let Tokens::Text(txt) = &step_inside_val(ident, "identifier").name {
                        traits.push(txt.to_string());
                    }
                }
                generics.push(GenericDecl {
                    identifier: get_ident(generic),
                    traits,
                })
            }
        }
        generics
    }
    fn get_token<'a>(node: &'a Node, ident: &'a str) -> &'a Tokens {
        return &step_inside_val(&node, ident).name;
    }
    fn step_inside_val<'a>(node: &'a Node, ident: &'a str) -> &'a Node {
        node.nodes.get(ident).unwrap().get_value()
    }
    fn try_step_inside_val<'a>(node: &'a Node, ident: &'a str) -> Option<&'a Node> {
        match node.nodes.get(ident) {
            Some(arr) => Some(arr.get_value()),
            None => None,
        }
    }
    fn step_inside_arr<'a>(node: &'a Node, ident: &'a str) -> &'a Vec<Node> {
        node.nodes.get(ident).unwrap().get_array()
    }
    fn try_step_inside_arr<'a>(node: &'a Node, ident: &'a str) -> Option<&'a Vec<Node>> {
        match node.nodes.get(ident) {
            Some(arr) => Some(arr.get_array()),
            None => None,
        }
    }
    /// all of the defined types/variables (enum, struct, function) in the current scope will be registered here
    #[derive(Debug)]
    pub struct Dictionary {
        pub functions: Vec<Function>,
        pub types: Vec<TypeDef>,
        pub enums: Vec<Enum>,
        pub structs: Vec<Struct>,
        pub variables: Vec<Variable>,
        pub identifiers: Vec<(String, IdentifierKinds)>,
        pub imports: Vec<Dictionary>,
    }
    #[derive(Debug)]
    pub enum IdentifierKinds {
        Function,
        Type,
        Enum,
        Struct,
        Variable,
        Namespace,
    }
    #[derive(Debug)]
    pub struct TypeDef {
        kind: ShallowType,
        identifier: String,
        generics: Vec<GenericDecl>,
    }
    #[derive(Debug)]
    pub struct GenericDecl {
        identifier: String,
        traits: Vec<String>,
    }
    #[derive(Debug)]
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
    }
    #[derive(Debug)]
    pub struct Enum {
        pub identifier: String,
        /// enum values and their offset
        /// enum ErrCode { Continue = 100, SwitchingProtocols, ..., Ok = 200, ... }
        pub keys: Vec<(String, usize)>,
        /// location in source code
        pub src_loc: usize,
        pub methods: Vec<Function>,
    }
    #[derive(Debug)]
    pub struct Struct {
        pub generics: Vec<GenericDecl>,
        pub identifier: String,
        pub keys: Vec<(String, ShallowType)>,
        /// location in source code
        pub src_loc: usize,
        pub methods: Vec<Function>,
    }
    #[derive(Debug)]
    pub struct Variable {
        pub kind: Types,
        pub identifier: String,
        /// location on stack
        pub location: usize,
    }
    /// identifiers can not contain these characters: + - * / = % ; : , . ({<[]>}) & | ! ? " '
    /// map: let i: Int = 32; i = i + 63;
    ///     - match {keyword? => keyword(?), value? => value(?)} => keyword(let), identifier("i"), match {: => Type, = => None} => Type(Int), operator(=), value(32);
    ///     - match {keyword? => keyword(?), value? => value} => value, value("i"), operator(=), value("i"), operator(+), value(63);
    #[derive(Debug)]
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
        Function(String, GenericExpr),
        Enum(String, GenericExpr),
        Struct(String, GenericExpr),
    }
    type GenericExpr = Vec<ShallowType>;

    #[derive(Debug)]
    pub struct ShallowType {
        refs: usize,
        main: Vec<String>,
        generics: GenericExpr,
    }

    impl Dictionary {
        pub fn new() -> Self {
            Dictionary {
                functions: vec![],
                types: vec![],
                enums: vec![],
                structs: vec![],
                variables: vec![],
                identifiers: vec![],
                imports: vec![],
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
        fn type_of(&self, idx: usize) -> &IdentifierKinds {
            &self.identifiers[idx].1
        }
        fn register_id(&mut self, name: String, kind: IdentifierKinds) -> bool {
            if self.contains(&name) {
                return false;
            }
            self.identifiers.push((name, kind));
            true
        }
        fn contains(&self, name: &String) -> bool {
            for id in &self.identifiers {
                if id.0 == *name {
                    return true;
                }
            }
            false
        }
    }
}
pub mod AnalyzationError {
    #[derive(Debug)]
    pub enum ErrType {
        /// assigned_number line col | occurs when you try to assign same number to two or more enum variants
        EnumVariantAssignedNumber(usize, (usize, usize)),
        /// variant_ident line col | occurs when you try to assign same identifier to two or more enum variants
        EnumVariantAssignedIdent(String, (usize, usize)),
        /// name | occurs when you try to assign same identifier twice
        ConflictingNames(String),
    }
}
