use super::{Token, TokenKind}; //親モジュールで定義しているから定義しなくてもよくね？, ダメみたい
use super::util;
use std::collections::VecDeque;

//トークン列に分解する
pub fn tokenize(s: &mut String) -> VecDeque<Token> { //有限状態オートマトンらしく書いてみたかったりする
    
    let mut v: VecDeque<Token> = VecDeque::new();

    while s.len() > 0 {
        let c = s.chars().nth(0).unwrap(); //sの先頭要素を取得. スライスで行うのはNG

        if c == ' ' { //空白をスキップ
            s.remove(0);
        }else if c == '+' { //足し算の時
            v.push_back(Token::new(TokenKind::ADD, None));
            s.remove(0);
        }else if c == '-' { //引き算の時
            v.push_back(Token::new(TokenKind::SUB, None));
            s.remove(0);
        }else if c == '*' {
            v.push_back(Token::new(TokenKind::MUL, None));
            s.remove(0);
        }else if c == '/' {
            v.push_back(Token::new(TokenKind::DIV, None));
            s.remove(0);
        }else if c == '(' {
            v.push_back(Token::new(TokenKind::LPAR, None));
            s.remove(0);
        }else if c == ')' {
            v.push_back(Token::new(TokenKind::RPAR, None));
            s.remove(0);
        }else if c.is_numeric() { //数字の時
            v.push_back(Token::new(TokenKind::TKNUM, util::get_digit(s))); //get_digitで削除までしてくれる
        }else {
            eprintln!("トークナイズできません");
        }  
    }
    v.push_back(Token::new(TokenKind::TKEOF, None));
    v
}