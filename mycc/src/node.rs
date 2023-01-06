pub mod util;
pub mod stack_assembly;

// use std::collections::VecDeque;
// use crate::token::{Token, TokenKind};

#[derive(Debug, PartialEq)]
//列挙体はpubにすればメンバも自動的にpubになる
pub enum NodeKind {
    NDADD, // +
    NDSUB, // -
    NDMUL, // *
    NDDIV, // /
    NDNUM, //整数
}

//構文木を定義する列挙体
#[derive(Debug)]
pub enum Node {
    Nil,
    Elm {
        kind: NodeKind,
        lhs: Box<Node>, //Nodeのポインタを渡す
        rhs: Box<Node>, //Nodeのポインタを渡す
        val: Option<i32>,
    }
}

impl Node {
    //数値を持たないノードの追加
    pub fn new_node(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) ->Box<Node> {
        Box::new(Node::Elm { kind: kind, lhs: lhs, rhs: rhs, val: None })
    }

    //数値をもつノードの追加
    pub fn new_node_num(val: i32) -> Box<Node> {
        Box::new(Node::Elm { kind: NodeKind::NDNUM, lhs: Box::new(Node::Nil), rhs: Box::new(Node::Nil), val: Some(val) })
    }
}