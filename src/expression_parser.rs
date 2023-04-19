use core::panic;
use std::fmt::format;

use crate::intermediate::dictionary::*;
use crate::intermediate::AnalyzationError::ErrType;
use crate::lexer::tokenizer::{Operators, Tokens};
use crate::tree_walker::tree_walker::Node;
use crate::{intermediate, lexer};
use intermediate::dictionary::*;
use intermediate::*;

pub fn expr_into_tree(node: &Node, errors: &mut Vec<ErrType>) -> ValueType {
    let nodes = step_inside_arr(&node, "nodes");
    if nodes.len() == 0 {
        return ValueType::Expression(Box::new(ExprNode::blank()));
    }
    if let Tokens::Text(str) = &nodes[0].name {
        if str == "anonymous_function" {
            return ValueType::AnonymousFunction(get_fun_siginifier(&nodes[0], errors));
        }
    }
    let mut transform = transform_expr(&nodes, errors);
    let res = list_into_tree(&mut transform);
    if let Ok(val) = res {
        return val;
    } else {
        println!("error occured while parsing expression: {:?}", res);
        unreachable!("Blank expression parse error")
    }
}

/// %/*-+<>==!=<=>=&&||
const ORDER_OF_OPERATIONS: [Operators; 13] = [
    Operators::Mod,
    Operators::Slash,
    Operators::Star,
    Operators::Plus,
    Operators::Minus,
    Operators::AngleBracket(false),
    Operators::AngleBracket(true),
    Operators::Equal,
    Operators::NotEqual,
    Operators::LessEq,
    Operators::MoreEq,
    Operators::And,
    Operators::Or,
];

/// recursive function that transforms list of values into tree
pub fn list_into_tree(list: &mut Vec<ValueType>) -> Result<ValueType, TreeTransformError> {
    let mut result = ExprNode {
        left: None,
        right: None,
        operator: None,
    };
    if list.len() == 0 {
        return Ok(ValueType::Blank);
    }
    if list.len() == 1 {
        if let ValueType::Operator(_) = &list[0] {
            return Err(TreeTransformError::ExcessOperator);
        }
        return Ok(list.pop().unwrap());
    }
    for op in &ORDER_OF_OPERATIONS {
        let mut i = 0;
        // if the list consists of only 1 value and it is not an operator, return it
        while i < list.len() {
            if let ValueType::Operator(op2) = &list[i] {
                if op == op2 {
                    if i == 0 {
                        return Err(TreeTransformError::NoValue);
                    }
                    if i == list.len() - 1 {
                        return Err(TreeTransformError::ExcessOperator);
                    }
                    result.operator = Some(*op);
                    // split list into 2 lists using index
                    list.remove(i);
                    let mut right = list.split_off(i);
                    // call this function recursively for each side
                    // left side
                    if list.len() == 0 {
                        return Err(TreeTransformError::NoValue);
                    }
                    let res = list_into_tree(list);
                    if let Ok(left) = res {
                        result.left = Some(left);
                    } else {
                        return res;
                    }
                    // right side
                    if right.len() == 0 {
                        return Err(TreeTransformError::NoValue);
                    }
                    let res = list_into_tree(&mut right);
                    if let Ok(right) = res {
                        result.right = Some(right);
                    } else {
                        return res;
                    }

                    // return result
                    return Ok(ValueType::Expression(Box::new(result)));
                }
            }
            i += 1;
        }
    }
    return Err(TreeTransformError::NotImplementedCuzLazy);
}

#[derive(Debug)]
pub enum TreeTransformError {
    NoValue,
    ExcessOperator,
    ExcessValue,
    NotImplementedCuzLazy,
}

pub fn transform_expr(nodes: &Vec<Node>, errors: &mut Vec<ErrType>) -> Vec<ValueType> {
    let mut result = vec![];
    for node in nodes {
        if let Some(op) = try_get_op(&node, errors) {
            result.push(ValueType::Operator(op));
            continue;
        }
        if let Some(val) = try_get_value(&node, errors) {
            result.push(val);
            continue;
        }
    }
    result
}

pub fn try_get_value(node: &Node, errors: &mut Vec<ErrType>) -> Option<ValueType> {
    if let Tokens::Text(txt) = &node.name {
        if txt != "value" {
            return None;
        }
    }
    let prepend = get_prepend(step_inside_val(&node, "prepend"), errors);
    if let Some(car) = try_get_variable(step_inside_val(&node, "value"), errors) {
        return Some(ValueType::Value(Variable {
            unary: prepend.2,
            refs: prepend.0,
            modificatior: prepend.1,
            root: car.0,
            tail: car.1,
        }));
    }
    if let Some(lit) = try_get_literal(step_inside_val(&node, "value"), errors, &prepend) {
        return Some(ValueType::Literal(lit));
    }
    if let Some(paren) = try_get_parenthesis(step_inside_val(&node, "value"), errors) {
        return Some(ValueType::Parenthesis(Box::new(paren.0), paren.1));
    }
    None
}

pub fn try_get_literal(
    node: &Node,
    errors: &mut Vec<ErrType>,
    prepend: &(usize, Option<String>, Option<Operators>),
) -> Option<Literal> {
    if let Tokens::Text(txt) = &node.name {
        if txt != "literal" {
            return None;
        }
    }
    let this = step_inside_val(&node, "value");
    if let Tokens::Number(_, _, _) = &this.name {
        return Some(Literal {
            unary: prepend.2,
            refs: prepend.0,
            modificatior: prepend.1.clone(),
            value: Literals::Number(this.name.clone()),
        });
    }
    if let Tokens::String(str) = &this.name {
        return Some(Literal {
            unary: prepend.2,
            refs: prepend.0,
            modificatior: prepend.1.clone(),
            value: Literals::String(str.clone()),
        });
    }
    None
}

pub fn try_get_variable(
    node: &Node,
    errors: &mut Vec<ErrType>,
) -> Option<(String, Vec<TailNodes>)> {
    if let Tokens::Text(txt) = &node.name {
        if txt != "variable" {
            return None;
        }
    }
    let ident = get_ident(&node);
    let tail = get_tail(step_inside_val(&node, "tail"), errors);
    Some((ident, tail))
}

pub fn get_args(node: &Node, errors: &mut Vec<ErrType>) -> Vec<ValueType> {
    let mut result = vec![];
    for child in step_inside_arr(&node, "expressions") {
        result.push(expr_into_tree(&child, errors));
    }
    result
}

pub fn try_get_parenthesis(
    node: &Node,
    errors: &mut Vec<ErrType>,
) -> Option<(ValueType, Vec<TailNodes>)> {
    if let Tokens::Text(txt) = &node.name {
        if txt != "free_parenthesis" {
            return None;
        }
    }
    let tail = get_tail(step_inside_val(&node, "tail"), errors);
    let expression = expr_into_tree(step_inside_val(&node, "expression"), errors);
    Some((expression, tail))
}

pub fn get_tail(node: &Node, errors: &mut Vec<ErrType>) -> Vec<TailNodes> {
    let mut tail = vec![];
    for child in step_inside_arr(&node, "nodes") {
        if let Tokens::Text(txt) = &child.name {
            if txt == "idx" {
                let expr = expr_into_tree(step_inside_val(&child, "expression"), errors);
                tail.push(TailNodes::Index(expr));
                continue;
            }
            if txt == "nested" {
                tail.push(TailNodes::Nested(get_ident(&child)));
                continue;
            }
            if txt == "function_call" {
                let generic = get_generics_expr(&child, errors);
                let args = get_args(step_inside_val(&child, "parenthesis"), errors);
                tail.push(TailNodes::Call(FunctionCall { generic, args }));
                continue;
            }
            if txt == "cast" {
                let kind = get_nested_ident(step_inside_val(&child, "type"), errors);
                tail.push(TailNodes::Cast(kind));
                break;
            }
        }
    }
    tail
}

pub fn get_prepend(
    node: &Node,
    errors: &mut Vec<ErrType>,
) -> (usize, Option<String>, Option<Operators>) {
    let refs = count_refs(&node);
    let modificator = if let Tokens::Text(txt) = &step_inside_val(&node, "keywords").name {
        if txt == "'none" {
            None
        } else {
            Some(txt.to_string())
        }
    } else {
        None
    };
    let unary = if let Some(un) = try_step_inside_val(&step_inside_val(&node, "unary"), "op") {
        if let Tokens::Operator(op) = un.name {
            Some(op)
        } else {
            None
        }
    } else {
        None
    };
    (refs, modificator, unary)
}

pub fn try_get_op(node: &Node, errors: &mut Vec<ErrType>) -> Option<Operators> {
    if let Tokens::Text(txt) = &node.name {
        if txt != "operator" {
            return None;
        }
    }
    if let Tokens::Operator(op) = step_inside_val(&node, "op").name {
        return Some(op);
    }
    None
}

#[derive(Debug)]
pub struct ExprNode {
    left: Option<ValueType>,
    right: Option<ValueType>,
    operator: Option<Operators>,
}
impl ExprNode {
    pub fn new(
        left: Option<ValueType>,
        right: Option<ValueType>,
        operator: Option<Operators>,
    ) -> ExprNode {
        ExprNode {
            left,
            right,
            operator,
        }
    }
    pub fn blank() -> ExprNode {
        ExprNode {
            left: None,
            right: None,
            operator: None,
        }
    }
}
#[derive(Debug)]
pub enum ValueType {
    Literal(Literal),
    AnonymousFunction(Function),
    /// parenthesis
    Parenthesis(Box<ValueType>, Vec<TailNodes>),
    Expression(Box<ExprNode>),
    /// only for inner functionality
    Operator(Operators),
    Value(Variable),
    Blank,
}
impl ValueType {
    pub fn fun(fun: Function) -> ValueType {
        ValueType::AnonymousFunction(fun)
    }
    pub fn value(val: Literal) -> ValueType {
        ValueType::Literal(val)
    }
}
#[derive(Debug)]
pub struct Literal {
    unary: Option<Operators>,
    refs: usize,
    /// atm only keyword new, so bool would be sufficient, but who knows what will be in the future updates
    modificatior: Option<String>,
    value: Literals,
}
#[derive(Debug)]
pub enum Literals {
    Number(Tokens),
    Array(ArrayRule),
    String(String),
}
#[derive(Debug)]
pub enum ArrayRule {
    Fill(Box<ExprNode>, usize),
    Explicit(Vec<ExprNode>),
}
#[derive(Debug)]
pub struct Variable {
    unary: Option<Operators>,
    refs: usize,
    /// atm only keyword new, so bool would be sufficient, but who knows what will be in the future updates
    modificatior: Option<String>,
    /// for longer variables
    /// example: danda[5].touch_grass(9)
    ///          ~~~~~ <- this is considered a root
    root: String,
    /// for longer variables
    /// example: danda[5].touch_grass(9)
    /// danda is root .. rest is tail
    tail: Vec<TailNodes>,
}

#[derive(Debug)]
pub struct FunctionCall {
    generic: Vec<ShallowType>,
    args: Vec<ValueType>,
}
#[derive(Debug)]
pub enum TailNodes {
    Nested(String),
    Index(ValueType),
    Call(FunctionCall),
    Cast(Vec<String>),
}

pub fn try_is_operator(node: &Node, errors: &mut Vec<ErrType>) -> Option<Operators> {
    if let Tokens::Operator(op) = &node.name {
        return Some(*op);
    }
    None
}
