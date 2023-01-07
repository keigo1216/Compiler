use super::{Token, TokenKind}; //親モジュールで定義しているから定義しなくてもよくね？, ダメみたい
use super::util;
use std::collections::VecDeque;

//トークン列に分解する
pub fn tokenize(s: &mut String) -> VecDeque<Token> { //有限状態オートマトンらしく書いてみたかったりする
    
    let mut v: VecDeque<Token> = VecDeque::new();

    while s.len() > 0 {
        let c = s.chars().nth(0).unwrap();

        match c {
            ' ' => {
                s.remove(0);
            }
            '+' => {
                v.push_back(Token::new(TokenKind::ADD, None));
                s.remove(0);
            }
            '-' => {
                v.push_back(Token::new(TokenKind::SUB, None));
                s.remove(0);
            }
            '*' => {
                v.push_back(Token::new(TokenKind::MUL, None));
                s.remove(0);
            }
            '/' => {
                v.push_back(Token::new(TokenKind::DIV, None));
                s.remove(0);
            }
            '(' => {
                v.push_back(Token::new(TokenKind::LPAR, None));
                s.remove(0);
            }
            ')' => {
                v.push_back(Token::new(TokenKind::RPAR, None));
                s.remove(0);
            }
            x if x.is_numeric() => {
                v.push_back(Token::new(TokenKind::TKNUM, util::get_digit(s))); //get_digitで削除までしてくれる
            }
            _ => {
                eprintln!("トークナイズできません");
            }
        }
    }

    v.push_back(Token::new(TokenKind::TKEOF, None));
    v
}