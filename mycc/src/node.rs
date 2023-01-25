pub mod generative_rule;
pub mod util;

//()は演算ではないから構文木の種類には入らない
#[derive(Debug, PartialEq)]
//列挙体はpubにすればメンバも自動的にpubになる
pub enum NodeKind {
    NDADD, // +
    NDSUB, // -
    NDMUL, // *
    NDDIV, // /
    NDASS, //= 代入文
    NDEQ,  //==
    NDNEQ, //ノットイコール
    NDLT,  //<
    NDLE,  //<=
    NDGT,  //>
    NDGE,  //>=
    NDLVAR, //ローカル変数
    NDNUM, //整数
    NDRETURN, //return
}

//構文木を定義する列挙体
#[derive(Debug)]
pub enum Node {
    Nil,
    Elm {
        kind: NodeKind,
        lhs: Box<Node>, //Nodeのポインタを渡す
        rhs: Box<Node>, //Nodeのポインタを渡す
        val: Option<i32>, //NDNUMの値
        offset: Option<i32>, //NDLVARのときのベースアドレスからのオフセット
    }
}

impl Node {
    //数値を持たないノードの追加
    pub fn new_node(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) ->Box<Node> {
        Box::new(Node::Elm { kind: kind, lhs: lhs, rhs: rhs, val: None, offset: None})
    }

    //数値をもつノードの追加
    pub fn new_node_num(val: i32) -> Box<Node> {
        Box::new(Node::Elm { kind: NodeKind::NDNUM, lhs: Box::new(Node::Nil), rhs: Box::new(Node::Nil), val: Some(val), offset: None})
    }

    //変数を持つノードの追加
    pub fn new_node_var(offset: i32) -> Box<Node> {
        Box::new(Node::Elm { kind: NodeKind::NDLVAR, lhs: Box::new(Node::Nil), rhs: Box::new(Node::Nil), val: None, offset: Some(offset)})
    }
}