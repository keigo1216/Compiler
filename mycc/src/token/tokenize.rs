use super::{Token, TokenKind}; //親モジュールで定義しているから定義しなくてもよくね？, ダメみたい
use super::util;
use std::collections::VecDeque;

//トークン列に分解する
pub fn tokenize(s: &mut String) -> VecDeque<Token> { //有限状態オートマトンらしく書いてみたかったりする
    
    let mut v: VecDeque<Token> = VecDeque::new();

    while s.len() > 0 {
        // let c = s.chars().nth(0).unwrap();

        //予約語のマッチ
        if util::consume_return(s) { //returnトークン
            s.replace_range(0..6, ""); //先頭の6文字を削除する
            v.push_back(Token::new(TokenKind::RETURN, None, None));
            continue;
        }

        //一文字のパターンマッチ
        let c = s.chars().nth(0).unwrap();
        match c {
            ' ' => {
                s.remove(0);
            }
            x if x.is_alphabetic() => { //cがアルファベットの時
                // s.remove(0);
                v.push_back(Token::new(TokenKind::ID, util::get_id_name(s), None)); //get_id_nameでremoveまでしてくれる
            }
            '+' => {
                s.remove(0);
                v.push_back(Token::new(TokenKind::ADD, None,None));
            }
            '-' => {
                s.remove(0);
                v.push_back(Token::new(TokenKind::SUB, None, None));
            }
            '*' => {
                s.remove(0);
                v.push_back(Token::new(TokenKind::MUL, None, None));
            }
            '/' => {
                s.remove(0);
                v.push_back(Token::new(TokenKind::DIV, None, None));
            }
            '=' => {//代入文かboolen文なのかを判断する
                s.remove(0);
                v.push_back(Token::new(util::judge_equal_symbol_token(s), None, None));
            }
            '!' => {//ノットイコールになっているのかを判断する
                s.remove(0);
                v.push_back(Token::new(util::judge_no_equal_symbol_token(s), None, None));
            }
            '<' => {//<なのか<=なのかを判断する, 最長一致が基本
                s.remove(0);
                v.push_back(Token::new(util::judge_less_symbol_token(s), None, None));
            }
            '>' => {//>なのか>=なのかを判断する. 最長一致が基本
                s.remove(0);
                v.push_back(Token::new(util::judge_greater_symbol_token(s), None, None));
            }
            '(' => {
                s.remove(0);
                v.push_back(Token::new(TokenKind::LPAR, None, None));
            }
            ')' => {
                s.remove(0);
                v.push_back(Token::new(TokenKind::RPAR, None, None));
            }
            x if x.is_numeric() => {
                v.push_back(Token::new(TokenKind::TKNUM, None,util::get_digit(s))); //get_digitで削除までしてくれる
            }
            ';' => {//セミコロンの時
                s.remove(0);
                v.push_back(Token::new(TokenKind::SEMI, None, None));
            }
            _ => {
                eprintln!("{} is not match the token pattern. ", c);
                std::process::exit(1);
            }
        }
    }

    v.push_back(Token::new(TokenKind::TKEOF, None,None));
    v
}