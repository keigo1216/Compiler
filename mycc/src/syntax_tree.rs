use std::collections::VecDeque;
mod function;
mod generative_rule;
mod node;
mod util;

//()は演算ではないから構文木の種類には入らない
#[derive(Debug, PartialEq, Clone)]
//列挙体はpubにすればメンバも自動的にpubになる
pub enum NodeKind {
    NDHEAD, // headを表す, 特別な意味がない
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
    NDIF, //if
    NDFOR, //for
    NDBLOCK, //{}
}

//構文木を定義する列挙体
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Nil,
    Elm {
        kind: NodeKind,
        lhs: Box<Node>, //Nodeのポインタを渡す
        rhs: Box<Node>, //Nodeのポインタを渡す
        val: Option<i32>, //NDNUMの値
        offset: Option<i32>, //NDLVARのときのベースアドレスからのオフセット

        // if文, for文
        cond: Box<Node>,
        then: Box<Node>,
        els: Box<Node>,
        init: Box<Node>,
        inc: Box<Node>,

        // ブロック
        body: VecDeque<Box<Node>>
    }
}

//プログラムファイルで宣言されたローカル変数を扱う
pub struct Obj {
    name: String, //ローカル変数
    offset: i32, //ベースポインタからのオフセット
}

// Function
pub struct Function {
    pub body: VecDeque<Box<Node>>, // 構文木を格納
    pub locals: Vec<Obj>, //ローカル変数のベースポインタからのオフセット
    pub stack_size: i32 //プロローグで必要なスタックサイズ
}

//生成規則を定義
pub struct GenerativeRule{}