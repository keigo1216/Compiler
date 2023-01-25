use crate::token::{Token, TokenKind};
use std::collections::VecDeque;

//VecDequeの先頭要素のKindがopと一致したらTrue, それ以外はFalseを返す
pub fn consume(token: &mut VecDeque<Token>, op: TokenKind) -> bool{
    let front_token = token.pop_front();

    if let Some(t) = front_token { //front_tokenがSome()だったら実行、中身がtに入る
        if t.kind == op {true}
        else {
            token.push_front(t);
            false
        }
    }else{ //front_tokenがNoneだったらエラー
        eprintln!("expect have an element, but it's empty. ");
        std::process::exit(1);
    }
}

//先頭のトークンがIDトークンかどうか
//実際に取り出したりしない
pub fn consume_ident(token: &mut VecDeque<Token>) -> bool {
    let front_token = token.pop_front(); //先頭のトークンを取得

    match front_token {
        Some(t @ Token { kind:TokenKind::ID, ..}) => {
            token.push_front(t);
            return true;
        }
        Some(t) => {
            token.push_front(t);
            return false;
        }
        None => {
            eprintln!("expect have an element, but it's empty. ");
            std::process::exit(1);
        }
    }
}

//primary関数で使う
//この関数はだいぶ汚いので綺麗に書き直したい
//トークンがIDならStringを返す
//それ以外の時はエラーを返す
//consume_identと一緒に使ってください
//エラーが出るから変なバグには繋がらないと思います
//半分ぐらいconsume_identと同じことしているのでまとめたいところ
pub fn expect_id(token: &mut VecDeque<Token>) -> String {

    let front_token = token.pop_front();

    match front_token {
        Some(Token { kind:TokenKind::ID, str:Some(s), ..}) => { //正常なとき
            return s;
        }
        Some(Token { kind: TokenKind::ID, str:None, ..}) => { //ノードのトークンはIDだが文字列が入っていないとき
            eprintln!("Node token is ID, but don't have an ID String. ");
            std::process::exit(1);
        }
        Some(_) => { //ID以外のノードを持つのが入ってきたとき
            eprintln!("expect have ID token, but don't have. ");
            std::process::exit(1);
        }
        None => { //ノードがNoneのとき
            eprintln!("expect have an element, but it's empty. ");
            std::process::exit(1);
        }
    }
}

//VecDequeの先頭要素が数字の時その先頭要素を返し、それ以外の時エラー出力する
pub fn expect_number(token: &mut VecDeque<Token>) -> i32{
    let front_token = token.pop_front();
    
    match front_token {
        Some(Token { kind: TokenKind::TKNUM, val: Some(_val), ..}) => { //正常なとき
            return _val;
        }
        Some(Token{ kind: TokenKind::TKNUM, val: None, ..}) => { //トークンはNUMだが、数字が空の時
            eprintln!("Node token is TKNUM, but don't have an val. ");
            std::process::exit(1);
        }
        Some(_) => {
            eprintln!("expect have TKNUM, but don't have. ");
            std::process::exit(1);
        }
        None => {
            eprintln!("expect have an element, but it's empty. ");
            std::process::exit(1);
        }
    }
}

//VecDequeの先頭要素のKindがopと一致していなかったらエラー出力する
//全てのパターンに一致しなかったトークン用
pub fn expect(token: &mut VecDeque<Token>, op: TokenKind) {
    let front_token = token.pop_front();

    if let Some(t) = front_token {
        if t.kind != op {
            eprintln!("expect have {:?}, but you have {:?}. ", op, t.kind); 
            std::process::exit(1); 
        }
    }else {
        eprintln!("expect have an element, but it's empty. ");
        std::process::exit(1);
    }

}

pub fn at_eof(token: &mut VecDeque<Token>) -> bool {
    let front_token = token.pop_front();

    match front_token {
        Some(Token{ kind:TokenKind::TKEOF, ..}) => { //コードの終端のとき
            return true;
        }
        Some(t) => { //終端ではなかったとき
            token.push_front(t);
            return false;
        }
        None => { //トークンが存在しなかったとき
            eprintln!("expect have an element, but it's empty. ");
            std::process::exit(1);
        }
    }
}
