use super::{Token, TokenKind};
use std::collections::VecDeque;

//VecDequeの先頭要素のKindがopと一致したらTrue, それ以外はFalseを返す
pub fn consume(token: &mut VecDeque<Token>, op: TokenKind) -> bool{
    let front_token = token.pop_front();

    match front_token {
        Some(t) => {
            if t.kind == op {
                true
            }else{
                token.push_front(t);
                false
            }
        }
        None => {
            eprintln!("空です");
            std::process::exit(1);
        }
    }
}

//VecDequeの先頭要素が数字の時その先頭要素を返し、それ以外の時エラー出力する
pub fn expect_number(token: &mut VecDeque<Token>) -> i32{
    let front_token = token.pop_front();
    
    // ネストしすぎなので綺麗に書く方法ないですかね
    match front_token { //VecDequeが空ならNoneが帰ってくるからそれを弾く
        Some(t) => {
            if t.kind == TokenKind::TKNUM { //t.kindが数字の時, 比較するにはTokenKindにPartialEqを実装する
                match t.val { //valが存在したらその値を返し、存在しなかったらエラー
                    Some(x) => x,
                    None => {
                        eprintln!("数字が有効ではありません");
                        std::process::exit(1);
                    }
                }
            }else {
                eprintln!("先頭が数字ではありません");
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("空です");
            std::process::exit(1); 
        }
    }

}

//VecDequeの先頭要素のKindがopと一致していなかったらエラー出力する
//全てのパターンに一致しなかったトークン用
pub fn expect(token: &mut VecDeque<Token>, op: TokenKind) {
    let front_token = token.pop_front();

    match front_token {
        Some(t) => {
            if t.kind != op {
                eprintln!("対応するトークンが存在しません"); //Displayを実装する
                std::process::exit(1); 
            }
        }
        None => {
            eprintln!("空です");
            std::process::exit(1);
        }
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