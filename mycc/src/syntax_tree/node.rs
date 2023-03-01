use std::collections::VecDeque;

use super::{Node, NodeKind};

impl Node {
    //数値を持たないノードの追加
    pub fn new_node(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) ->Box<Node> {
        Box::new(Node::Elm { 
            kind, 
            lhs, 
            rhs, 
            val: None, 
            offset: None, 
            cond: Box::new(Node::Nil), 
            then: Box::new(Node::Nil), 
            els: Box::new(Node::Nil),
            init: Box::new(Node::Nil),
            inc: Box::new(Node::Nil), 
            body: VecDeque::new()
        })
    }

    //数値をもつノードの追加
    pub fn new_node_num(val: i32) -> Box<Node> {
        Box::new(Node::Elm { 
            kind: NodeKind::NDNUM, 
            lhs: Box::new(Node::Nil), 
            rhs: Box::new(Node::Nil), 
            val: Some(val), 
            offset: None, 
            cond: Box::new(Node::Nil), 
            then: Box::new(Node::Nil), 
            els: Box::new(Node::Nil), 
            init: Box::new(Node::Nil),
            inc: Box::new(Node::Nil), 
            body: VecDeque::new()
        })
    }

    //変数を持つノードの追加
    pub fn new_node_var(offset: i32) -> Box<Node> {
        Box::new(Node::Elm { 
            kind: NodeKind::NDLVAR, 
            lhs: Box::new(Node::Nil), 
            rhs: Box::new(Node::Nil), 
            val: None, 
            offset: Some(offset), 
            cond: Box::new(Node::Nil), 
            then: Box::new(Node::Nil), 
            els: Box::new(Node::Nil), 
            init: Box::new(Node::Nil),
            inc: Box::new(Node::Nil), 
            body: VecDeque::new()
        })
    }

    //if文をもつノードの追加
    pub fn new_node_if(cond: Box<Node>, then: Box<Node>, els: Box<Node>) -> Box<Node> {
        Box::new(Node::Elm { 
            kind: NodeKind::NDIF, 
            lhs: Box::new(Node::Nil), 
            rhs: Box::new(Node::Nil), 
            val: None, 
            offset: None, 
            cond, 
            then, 
            els, 
            init: Box::new(Node::Nil),
            inc: Box::new(Node::Nil), 
            body: VecDeque::new()
        })
    }

    //for文を持つノードの追加
    pub fn new_node_for(init: Box<Node>, cond: Box<Node>, inc: Box<Node>, then: Box<Node>) -> Box<Node> {
        Box::new(Node::Elm { 
            kind: NodeKind::NDFOR, 
            lhs: Box::new(Node::Nil), 
            rhs: Box::new(Node::Nil), 
            val: None, 
            offset: None, 
            cond, 
            then, 
            els: Box::new(Node::Nil), 
            init, 
            inc, 
            body: VecDeque::new() 
        })
    }

    pub fn new_node_while (cond: Box<Node>, then: Box<Node>) -> Box<Node> {
        Node::new_node_for(Box::new(Node::Nil), cond, Box::new(Node::Nil), then)
    }

    //ブロックを持つノードの追加
    pub fn new_node_block(body: VecDeque<Box<Node>>) -> Box<Node> {
        Box::new(Node::Elm { 
            kind: NodeKind::NDBLOCK, 
            lhs: Box::new(Node::Nil), 
            rhs: Box::new(Node::Nil), 
            val: None, 
            offset: None, 
            cond: Box::new(Node::Nil), 
            then: Box::new(Node::Nil), 
            els: Box::new(Node::Nil), 
            init: Box::new(Node::Nil),
            inc: Box::new(Node::Nil), 
            body
        })
    }
}