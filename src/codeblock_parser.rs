use crate::intermediate::AnalyzationError::ErrType;
use crate::{expression_parser::*, tree_walker};
use crate::intermediate::dictionary::{ShallowType, step_inside_arr};
use crate::lexer::tokenizer::*;
use crate::type_check::*;

pub fn generate_tree(node: &tree_walker::tree_walker::Node, errors: &mut Vec<ErrType>) -> Vec<Nodes> {
    if let Tokens::Text(txt) = &node.name {
        if txt != "code_block" {
            errors.push(ErrType::NotCodeBlock);
            return Vec::new();
        }
    }else {
        errors.push(ErrType::NotCodeBlock);
        return Vec::new();
    }
    let mut nodes = Vec::new();
    for node in step_inside_arr(&node, "nodes") {
        let temp = node_from_node(node, errors);
        if let Some(temp) = temp {
            nodes.push(temp);
        }
    }
    nodes
}

pub fn node_from_node(node: &tree_walker::tree_walker::Node, errors: &mut Vec<ErrType>) -> Option<Nodes> {
    if let Tokens::Text(txt) = &node.name {
        match txt.as_str() {
            "KWReturn" => {
                Some(Nodes::Return {
                    expr: None
                })
            }
            _ => None
        }
    }else {
        None
    }
}

#[derive(Debug)]
pub enum Nodes {
    Let {
        ident: String,
        expr: Option<ValueType>,
        kind: Option<ShallowType>,
    },
    If {
        cond: ValueType,
        body: Vec<Nodes>,
        elif: Vec<Nodes>,
        els: Option<Vec<Nodes>>,
    },
    While {
        cond: ValueType,
        body: Vec<Nodes>,
    },
    For {
        ident: String,
        expr: ValueType,
        body: Vec<Nodes>,
    },
    Return {
        expr: Option<ValueType>,
    },
    Expr {
        expr: ValueType,
    },
    Block {
        body: Vec<Nodes>,
    },
    Break,
    Continue,
    Loop {
        body: Vec<Nodes>,
    },
    Yeet {
        expr: ValueType,
    },
    Try {
        body: Vec<Nodes>,
        ///     catches ((ident, [types]), body)
        catch: Vec<((String, Vec<String>), Vec<Nodes>)>,
        finally: Option<Vec<Nodes>>,
    },
    Switch {
        expr: ValueType,
        body: Vec<(ValueType, Vec<Nodes>)>,
        default: Option<Vec<Nodes>>,
    },
    Set {
        target: ValueType,
        expr: ValueType,
    },
}