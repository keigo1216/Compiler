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

//VecDequeの先頭要素が数字の時その先頭要素を返し、それ以外の時エラー出力する
pub fn expect_number(token: &mut VecDeque<Token>) -> i32{
    let front_token = token.pop_front();
    
    //ネストが深くて読みいにくいからなんとかしたいけど...
    if let Some(t) = front_token {
        if let TokenKind::TKNUM = t.kind {
            if let Some(x) = t.val {x}
            else {
                eprintln!("数字が有効ではありません");
                std::process::exit(1);
            }
        }else{
            eprintln!("先頭が数字ではありません");
            std::process::exit(1);
        }
    }else{
        eprintln!("先頭が数字ではありません");
        std::process::exit(1);
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

//代入文かboolen文なのかを判断する
//適切なTokenKindを返す
//いらなくなった文字はこちらで削除する
pub fn judge_equal_symbol_token(s: &mut String) -> TokenKind {
    
    let c = s.chars().nth(0).unwrap();

    match c {
        '=' => {//==だった場合
            s.remove(0);
            return TokenKind::EQ;
        }
        _ => {//=だった場合
            return TokenKind::ASS;
        }
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