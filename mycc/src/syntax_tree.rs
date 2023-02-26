use std::collections::VecDeque;

mod syntax_tree;
mod generative_rule;
mod node;
mod util;

// use std::collections::VecDeque;
// use crate::token::Token;

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
    NDBLOCK, //{}
}

//構文木を定義する列挙体
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Nil,
    Elm {
        kind: NodeKind,
        next: Box<Node>, //次のstmtを管理する, デフォルトはNil
        lhs: Box<Node>, //Nodeのポインタを渡す
        rhs: Box<Node>, //Nodeのポインタを渡す
        val: Option<i32>, //NDNUMの値
        offset: Option<i32>, //NDLVARのときのベースアドレスからのオフセット

        // if文
        cond: Box<Node>,
        then: Box<Node>,
        els: Box<Node>,

        // ブロック
        body: VecDeque<Box<Node>>
    }
}

//プログラムファイルで宣言されたローカル変数を扱う
pub struct LVar {
    name: String, //ローカル変数
    offset: i32, //ベースポインタからのオフセット
}

// pub struct SyntaxTree {
//     vec_lvar: Vec<LVar>, //プログラム中のローカル変数のオフセットを格納
//     token: VecDeque<Token>, //字句解析で作成したトークンが入っている
//     code: VecDeque<Box<Node>>, //構文木を入れるキュー
// }

pub struct GenerativeRule{}