pub mod tokenize; //mainで使えるように公開する, サブモジュールとして宣言
pub mod util;
// use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    ID,     //今は一文字の変数を表す
    ADD,    //足し算の記号
    SUB,    //引き算の記号
    MUL,    //掛け算の記号
    DIV,    //割り算の記号
    ASS,    //=　代入
    EQ,     //==
    NEQ,    //ノットイコール
    LT,    //<
    LE,    //<=
    GT,    //>
    GE,    //>=
    TKNUM,  // 整数トークン
    LPAR,   //開きかっこ(
    RPAR,   //閉じかっこ)
    SEMI,   //セミコロン
    TKEOF,  // 入力の終わりを表すトークン
}

//同じモジュール内からはメンバをpubで指定しなくてもアクセスできる
#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    str: Option<String>,
    val: Option<i32>,
}

impl Token {
    pub fn new(kind: TokenKind, str:Option<String>, val: Option<i32>) -> Token{
        Token {
            kind,
            str, //IDの変数名を格納する
            val, //NUMの数を入れる時に使用
        }
    }
}
