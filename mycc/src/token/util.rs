// use super::TokenKind;
use crate::token::{TokenKind, Tokenize};

impl Tokenize {
    //sの先頭の数字を取得する
    //数字が一桁とは限らないことに注意
    pub(super) fn get_digit(s: &mut String) -> Option<i32> {

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

    //sの先頭の文字列を取得する
    //文字列の長さが1とは限らないことに注意
    pub(super) fn get_id_name(s: &mut String) -> Option<String> {

        let mut d = String::new();

        //先頭はアルファベットかアンダースコア
        let c = s.chars().nth(0).unwrap();
        if c.is_alphabetic() || c == '_' {
            d.push(c);
            s.remove(0);
        }

        //先頭以外はアルファベット、数字、アンダースコア
        while s.len() > 0 {
            let c = s.chars().nth(0).unwrap();

            if c.is_alphanumeric() || c == '_' {
                d.push(c);
                s.remove(0);
            } else {
                break;
            }
        }
        
        Some(d)
    }

    //先頭の文字がreturnかどうかを判断する
    //先頭の6文字がreturnで7文字目がアルファベット、数字、アンダースコア以外だとOK
    pub(super) fn is_return(s: &mut String) -> bool {

        if s.len() < 6 { return false; } //6文字より小さかったらそもそもだめ

        //先頭がreturnで始まっている
        if &s[0..6] == "return" { //先頭の6文字が"return"
            let c = s.chars().nth(6).unwrap(); //returnの次の文字を見る
            if c.is_alphanumeric() || c == '_' { //7文字目がアルファベット、数字、アンダースコア
                return false;
            } else { //それ以外
                return true;
            }
        } else { //先頭の6文字が"return"ではない
            return false;
        }
    }

    // if文かを判定する
    pub(super) fn is_if(s: &mut String) -> bool {

        if s.len() < 2 { return false; }

        if &s[0..2] == "if" { //先頭の2文字が"if"
            let c = s.chars().nth(2).unwrap(); //ifの次の文字を見る
            if c.is_alphanumeric() || c == '_' { //3文字目がアルファベット、数字、アンダースコア
                return false;
            } else { //それ以外
                return true;
            }
        } else { //先頭の二文字が"if"ではない
            return false;
        }
    }

    pub(super) fn is_for(s: &mut String) -> bool {

        if s.len() < 3 { return false; }

        if &s[0..3] == "for" {
            let c = s.chars().nth(3).unwrap();
            if c.is_alphanumeric() || c == '_' {
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }

    // else文かを判定する
    pub(super) fn is_else(s: &mut String) -> bool {

        if s.len() < 4 { return false; }
        
        if &s[0..4] == "else" {
            let c = s.chars().nth(4).unwrap();
            if c.is_alphanumeric() || c == '_' {
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }

    pub(super) fn is_while(s: &mut String) -> bool {

        if s.len() < 5 { return false; }

        if &s[0..5] == "while" {
            let c = s.chars().nth(5).unwrap();
            if c.is_alphanumeric() || c == '_' {
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }

    //代入文かboolen文なのかを判断する
    //適切なTokenKindを返す
    //いらなくなった文字はこちらで削除する
    pub(super) fn judge_equal_symbol_token(s: &mut String) -> TokenKind {
        
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
    pub(super) fn judge_no_equal_symbol_token(s: &mut String) -> TokenKind {

        let c = s.chars().nth(0).unwrap();

        if c == '=' { //ノットイコールの場合
            s.remove(0);
            return TokenKind::NEQ;
        }else{
            eprintln!("expect have =, but don't have. ");
            std::process::exit(1);
        }

    }

    //<記号が入ってきた時に'<'または'<='を判断する
    //小なり
    pub(super) fn judge_less_symbol_token(s: &mut String) -> TokenKind {

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
    pub(super) fn judge_greater_symbol_token(s: &mut String) -> TokenKind {

        let c = s.chars().nth(0).unwrap();

        if c == '=' {
            s.remove(0);
            return TokenKind::GE;
        }else{
            return TokenKind::GT;
        }

    }
}