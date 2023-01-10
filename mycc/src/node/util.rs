use std::collections::VecDeque;
use crate::token::{Token, TokenKind, util};
use crate::node::{Node, NodeKind};

//非終端記号

//一つのプログラムの塊は複数のステートメントで成り立っている
//生成規則
//program = stmt*
// *は一つ以上を表す正規表現
pub fn program(token: &mut VecDeque<Token>) -> VecDeque<Box<Node>> {
    let mut code: VecDeque<Box<Node>> = VecDeque::new();
    while !util::at_eof(token) {
        code.push_back(stmt(token));
    }
    code
}

//一つのステートメントの一番最後はセミコロンで書かれている
//生成規則
//stmt = expr ';'
pub fn stmt(token: &mut VecDeque<Token>) -> Box<Node> {
    let node = expr(token);
    util::expect(token, TokenKind::SEMI); //最後がセミコロンでない時はエラーを出す
    node
}

//これの生成規則が何を意味するのかがあんまりわからない
//生成規則
//expr = assign
pub fn expr(token: &mut VecDeque<Token>) -> Box<Node> {
    // equality(token)
    assign(token)
}

//assignは代入文、または通常の式を表現する
//生成規則
//assign = equality ("=" assign)?
pub fn assign(token: &mut VecDeque<Token>) -> Box<Node> {
    let mut node = equality(token);

    loop {
        if util::consume(token, TokenKind::ASS) { //= 代入文の時
            node = Node::new_node(NodeKind::NDASS, node, assign(token));
        }else{
            return node;
        }
    }
}

//==とノットイコールを生成する
//生成規則
//equality   = relational ("==" relational | "!=" relational)*
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

//比較演算子を生成する
//生成規則
//relational = add ("<" add | "<=" add | ">" add | ">=" add)*
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

//足し算、引き算の演算子を生成する
//生成規則
//add = mul ("+" mul | "-" mul)*
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

//掛け算、割り算の演算子を生成する
//生成規則
//mul = unary ("*" unary | "/" unary)*
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

//単項演算子を生成する
//数字の±のところ
//たぶんここの生成規則間違っている
//生成規則
//unary = ("+" | "-")? primary（たぶんこれ間違ってる）
//unary = ("+" | "-") unary | primary
//?は0か1つ
pub fn unary(token: &mut VecDeque<Token>) -> Box<Node> {
    if util::consume(token, TokenKind::ADD) {
        unary(token)
    }else if util::consume(token, TokenKind::SUB) {
        Node::new_node(NodeKind::NDSUB, Node::new_node_num(0), unary(token))
    }else{
        primary(token)
    }
}


//数字、識別子（変数名）、カッコを生成する
//生成規則
//primary = num | ident | "(" expr ")"
pub fn primary(token: &mut VecDeque<Token>) -> Box<Node> {
    if util::consume(token, TokenKind::LPAR) { //(の時
        let node = expr(token);
        util::expect(token, TokenKind::RPAR);
        return node;
    }else if util::consume_ident(token) { //IDの時, ここ汚すぎですね、後で変更するはず
        let s = util::expect_id(token);
        let c = s.chars().nth(0).unwrap();
        let x = c as i32;
        let y = 'a' as i32;
        return Node::new_node_var((x - y + 1)*8); //多分ここは後々変えていくところ
    }   
    
    else {
        return Node::new_node_num(util::expect_number(token)); //数字があれば数字を返す、なければエラーを出す
    }
}