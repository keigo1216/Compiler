pub mod tokenize; //mainで使えるように公開する, サブモジュールとして宣言
pub mod util;
// use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    ADD,    //足し算の記号
    SUB,    //引き算の記号
    MUL,    //掛け算の記号
    DIV,    //割り算の記号
    ASS,    //代入
    EQ,     //==
    NEQ,    //ノットイコール
    LT,    //<
    LE,    //<=
    GT,    //>
    GE,    //>=
    TKNUM,  // 整数トークン
    LPAR,   //開きかっこ(
    RPAR,   //閉じかっこ)
    TKEOF,  // 入力の終わりを表すトークン
}

//同じモジュール内からはメンバをpubで指定しなくてもアクセスできる
#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    val: Option<i32>,
}

impl Token {
    pub fn new(kind: TokenKind, val: Option<i32>) -> Token{
        Token {
            kind,
            val, //はじめはNoneで初期化する
        }
    }
}
