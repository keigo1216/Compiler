use super::{Token, TokenKind};
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
        eprintln!("空です");
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
            eprintln!("空です");
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
            eprintln!("ノードにIDの文字列がありません");
            std::process::exit(1);
        }
        Some(_) => { //ID以外のノードを持つのが入ってきたとき
            eprintln!("ノードがIDではありません");
            std::process::exit(1);
        }
        None => { //ノードがNoneのとき
            eprintln!("ノードが存在しません");
            std::process::exit(1);
        }
    }
}

//VecDequeの先頭要素が数字の時その先頭要素を返し、それ以外の時エラー出力する
pub fn expect_number(token: &mut VecDeque<Token>) -> i32{
    let front_token = token.pop_front();
    
    //ネストが深くて読みいにくいからなんとかしたいけど...
    match front_token {
        Some(Token { kind: TokenKind::TKNUM, val: Some(_val), ..}) => {
            return _val;
        }
        Some(Token{ kind: TokenKind::TKNUM, val: None, ..}) => {
            eprintln!("数字が有効ではありません");
            std::process::exit(1);
        }
        Some(_) => {
            eprintln!("先頭が数字ではありません");
            std::process::exit(1);
        }
        None => {
            eprintln!("ノードが存在しません");
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
            eprintln!("対応するトークンが存在しません"); //Displayを実装する
            std::process::exit(1); 
        }
    }else {
        eprintln!("空です");
        std::process::exit(1);
    }

}

//sの先頭の数字を取得する
//数字が一桁とは限らないことに注意
pub fn get_digit(s: &mut String) -> Option<i32> {

    let mut d = String::new();
    while s.len() > 0 {
        let c = s.chars().nth(0).unwrap();

        if c.is_numeric() {
            d.push(c);
            s.remove(0);
        }else {
            break;
        }
    }
    let num: i32 = d.parse().unwrap();
    Some(num)
}

pub fn at_eof(token: &mut VecDeque<Token>) -> bool {
    let front_token = token.pop_front();

    match front_token {
        Some(Token{ kind:TokenKind::TKEOF, ..}) => {
            return true;
        }
        Some(t) => {
            token.push_front(t);
            return false;
        }
        None => {
            eprintln!("トークンが存在しません");
            std::process::exit(1);
        }
    }
}

//代入文かboolen文なのかを判断する
//適切なTokenKindを返す
//いらなくなった文字はこちらで削除する
pub fn judge_equal_symbol_token(s: &mut String) -> TokenKind {
    
    let c = s.chars().nth(0).unwrap();

    if c == '=' {
        s.remove(0);
        return TokenKind::EQ;
    }else {
        return TokenKind::ASS;
    }
}

//ビックリマークが入ってきた時に、その後に=が続いているのか
//続いていたらノットイコール
//続いていなかったら対応するトークンがないのでエラー
pub fn judge_no_equal_symbol_token(s: &mut String) -> TokenKind {

    let c = s.chars().nth(0).unwrap();

    if c == '=' { //ノットイコールの場合
        s.remove(0);
        return TokenKind::NEQ;
    }else{
        eprintln!("マッチするトークンがありません");
        std::process::exit(1);
    }

}

//<記号が入ってきた時に'<'または'<='を判断する
//小なり
pub fn judge_less_symbol_token(s: &mut String) -> TokenKind {

    let c = s.chars().nth(0).unwrap();

    if c == '=' { //<=の場合
        s.remove(0);
        return TokenKind::LE;
    }else{ //<の場合
        return TokenKind::LT;
    }
}


//>記号が入ってきた時に'>'または'>='を判断する
//大なり
pub fn judge_greater_symbol_token(s: &mut String) -> TokenKind {

    let c = s.chars().nth(0).unwrap();

    if c == '=' {
        s.remove(0);
        return TokenKind::GE;
    }else{
        return TokenKind::GT;
    }

}