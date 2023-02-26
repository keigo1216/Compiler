pub mod tokenize; //mainで使えるように公開する, サブモジュールとして宣言
pub mod util;

#[derive(Debug, PartialEq, Clone)]
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
    LBLOCK, //開き中カッコ{
    RBLOCK, //閉じ中カッコ}
    SEMI,   //セミコロン
    TKEOF,  // 入力の終わりを表すトークン
    RETURN, //return
    IF, //if
    ELS, //else
}

//同じモジュール内からはメンバをpubで指定しなくてもアクセスできる
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub str: Option<String>,
    pub val: Option<i32>,
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
