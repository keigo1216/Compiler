use std::collections::VecDeque;
use crate::token::{Token, TokenKind, util};
use crate::node::{Node, NodeKind};


pub fn expr(token: &mut VecDeque<Token>) -> Box<Node> {
    // let mut node = mul(token); //selfで書ける気がしたけどダメでした
    
    // loop {
    //     if util::consume(token, TokenKind::ADD) { //ADDトークンの時
    //         node = Node::new_node(NodeKind::NDADD, node, mul(token));
    //     }else if util::consume(token, TokenKind::SUB) { //SUBトークンの時 
    //         node = Node::new_node(NodeKind::NDSUB, node, mul(token));
    //     }else {
    //         return node;
    //     }
    // }
    equality(token)
}

pub fn equality(token: &mut VecDeque<Token>) -> Box<Node> {
    let mut node = relational(token);

    loop {
        if util::consume(token, TokenKind::EQ) { //==
            node = Node::new_node(NodeKind::NDEQ, node, relational(token));
        }else if util::consume(token, TokenKind::NEQ) { //ノットイコール
            node = Node::new_node(NodeKind::NDNEQ, node, relational(token));
        }else{
            return node;
        }
    }
}

pub fn relational(token: &mut VecDeque<Token>) -> Box<Node> {
    let mut node = add(token);

    loop {
        if util::consume(token, TokenKind::LT) {
            node = Node::new_node(NodeKind::NDLT, node, add(token));
        }else if util::consume(token, TokenKind::LE) {
            node = Node::new_node(NodeKind::NDLE, node, add(token));
        }else if util::consume(token, TokenKind::GT) {
            node = Node::new_node(NodeKind::NDGT, node, add(token));
        }else if util::consume(token, TokenKind::GE) {
            node = Node::new_node(NodeKind::NDGE, node, add(token));
        }else{
            return node;
        }
    }
}

pub fn add (token: &mut VecDeque<Token>) -> Box<Node> {
    let mut node = mul(token);

    loop {
        if util::consume(token, TokenKind::ADD) {
            node = Node::new_node(NodeKind::NDADD, node, mul(token));
        }else if util::consume(token, TokenKind::SUB) {
            node = Node::new_node(NodeKind::NDSUB, node, mul(token));
        }else{
            return node;
        }
    }
}

pub fn mul(token: &mut VecDeque<Token>) -> Box<Node> {
    let mut node = unary(token);

    loop {
        if util::consume(token, TokenKind::MUL) {
            node = Node::new_node(NodeKind::NDMUL, node, unary(token));
        }else if util::consume(token, TokenKind::DIV) {
            node = Node::new_node(NodeKind::NDDIV, node, unary(token));
        }else{
            return node;
        }
    }
}

pub fn unary(token: &mut VecDeque<Token>) -> Box<Node> {
    if util::consume(token, TokenKind::ADD) {
        primary(token)
    }else if util::consume(token, TokenKind::SUB) {
        Node::new_node(NodeKind::NDSUB, Node::new_node_num(0), primary(token))
    }else{
        primary(token)
    }
}

pub fn primary(token: &mut VecDeque<Token>) -> Box<Node> {
    if util::consume(token, TokenKind::LPAR) {
        let node = expr(token);
        util::expect(token, TokenKind::RPAR);
        return node;
    }else {
        return Node::new_node_num(util::expect_number(token));
    }
}