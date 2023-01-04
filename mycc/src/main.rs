use std::env;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum TokenKind {
    ADD,    //足し算の記号
    SUB,    //引き算の記号
    TKNUM,  // 整数トークン
    TKEOF,  // 入力の終わりを表すトークン
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    val: Option<i32>,
}

impl Token {
    pub fn new(kind: TokenKind, val: Option<i32>) -> Token{
        Token {
            kind,
            val, //はじめはNoneで初期化する
        }
    }
}

//VecDequeの先頭要素のKindがopと一致したらTrue, それ以外はFalseを返す
fn consume(token: &mut VecDeque<Token>, op: TokenKind) -> bool{
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
fn expect_number(token: &mut VecDeque<Token>) -> i32{
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
fn expect(token: &mut VecDeque<Token>, op: TokenKind) {
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
fn get_digit(s: &mut String) -> Option<i32> {

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

//トークン列に分解する
fn tokenize(s: &mut String) -> VecDeque<Token> { //文字列の所有権はこっちに移る
    
    let mut v: VecDeque<Token> = VecDeque::new();

    while s.len() > 0 {
        let c = s.chars().nth(0).unwrap(); //sの先頭要素を取得. スライスで行うのはNG

        if c == ' ' { //空白をスキップ
            s.remove(0);
        }else if c == '+' { //足し算の時
            v.push_back(Token::new(TokenKind::ADD, None));
            s.remove(0);
        }else if c == '-' {
            v.push_back(Token::new(TokenKind::SUB, None));
            s.remove(0);
        }else if c.is_numeric() {
            v.push_back(Token::new(TokenKind::TKNUM, get_digit(s))); //get_digitで削除までしてくれる
        }else {
            eprintln!("トークナイズできません");
        }  
    }
    v.push_back(Token::new(TokenKind::TKEOF, None));
    v
}

fn main() {
    let mut argv: Vec<String> = env::args().collect(); //コマンドライン引数を受け取る
    let argc = argv.len();

    if argc != 2 {
        eprintln!("引数の数が正しくありません。"); //標準エラー出力
        std::process::exit(1); 
    }

    let mut token = tokenize(&mut argv[1]); //コマンドラインで受け取った文字列をトークン列に変換する
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", expect_number(&mut token)); //はじめは数字、それ以外の場合はエラー

    //tokenを保管しているキューが空になるまで
    while !token.is_empty() {
        if consume(&mut token, TokenKind::ADD) {
            println!("  add rax, {}", expect_number(&mut token));
            continue;
        }

        if consume(&mut token, TokenKind::SUB) {
            println!("  sub rax, {}", expect_number(&mut token));
            continue;
        }

        expect(&mut token, TokenKind::TKEOF);
    }

    println!("  ret");
}
