use std::collections::VecDeque;

use super::{Node, NodeKind};

impl Node {
    //数値を持たないノードの追加
    pub fn new_node(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) ->Box<Node> {
        Box::new(Node::Elm { kind, next:Box::new(Node::Nil),lhs, rhs, val: None, offset: None, cond: Box::new(Node::Nil), then: Box::new(Node::Nil), els: Box::new(Node::Nil), body: VecDeque::new()})
    }

    //数値をもつノードの追加
    pub fn new_node_num(val: i32) -> Box<Node> {
        Box::new(Node::Elm { kind: NodeKind::NDNUM, next:Box::new(Node::Nil), lhs: Box::new(Node::Nil), rhs: Box::new(Node::Nil), val: Some(val), offset: None, cond: Box::new(Node::Nil), then: Box::new(Node::Nil), els: Box::new(Node::Nil), body: VecDeque::new()})
    }

    //変数を持つノードの追加
    pub fn new_node_var(offset: i32) -> Box<Node> {
        Box::new(Node::Elm { kind: NodeKind::NDLVAR, next:Box::new(Node::Nil), lhs: Box::new(Node::Nil), rhs: Box::new(Node::Nil), val: None, offset: Some(offset), cond: Box::new(Node::Nil), then: Box::new(Node::Nil), els: Box::new(Node::Nil), body: VecDeque::new()})
    }

    //制御文をもつノードの追加
    pub fn new_node_if(cond: Box<Node>, then: Box<Node>, els: Box<Node>) -> Box<Node> {
        Box::new(Node::Elm { kind: NodeKind::NDIF, next:Box::new(Node::Nil), lhs: Box::new(Node::Nil), rhs: Box::new(Node::Nil), val: None, offset: None, cond, then, els, body: VecDeque::new()})
    }

    //ブロックを持つノードの追加
    pub fn new_node_block(body: VecDeque<Box<Node>>) -> Box<Node> {
        Box::new(Node::Elm { kind: NodeKind::NDBLOCK, next:Box::new(Node::Nil), lhs: Box::new(Node::Nil), rhs: Box::new(Node::Nil), val: None, offset: None, cond: Box::new(Node::Nil), then: Box::new(Node::Nil), els: Box::new(Node::Nil), body})
    }
}