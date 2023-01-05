use std::env;
use std::collections::VecDeque;


#[derive(Debug, PartialEq)]
pub enum NodeKind {
    ND_ADD, // +
    ND_SUB, // -
    ND_MUL, // *
    ND_DIV, // /
    ND_NUM, //整数
}

//構文木を定義する列挙体
#[derive(Debug)]
pub enum Node {
    Nil,
    Elm {
        kind: NodeKind,
        lhs: Box<Node>, //Nodeのポインタを渡す
        rhs: Box<Node>, //Nodeのポインタを渡す
        val: Option<i32>,
    }
}

//構文木の先頭を表す
#[derive(Debug)]
pub enum Root {
    Empty,
    More(Box<Node>),
}

impl Root {
    pub fn new(node: Box<Node>) -> Root { //コンストラクタ
        Root::More(node)
    }
}

impl Node {
    //数値を持たないノードの追加
    pub fn new_node(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) ->Box<Node> {
        Box::new(Node::Elm { kind: kind, lhs: lhs, rhs: rhs, val: None })
    }

    //数値をもつノードの追加
    pub fn new_node_num(val: i32) -> Box<Node> {
        Box::new(Node::Elm { kind: NodeKind::ND_NUM, lhs: Box::new(Node::Nil), rhs: Box::new(Node::Nil), val: Some(val) })
    }

    pub fn expr(token: &mut VecDeque<Token>) -> Box<Node> {
        let mut node = Node::mul(token); //selfで書ける気がしたけどダメでした
        
        loop {
            if consume(token, TokenKind::ADD) { //ADDトークンの時
                node = Node::new_node(NodeKind::ND_ADD, node, Node::mul(token));
            }else if consume(token, TokenKind::SUB) { //SUBトークンの時 
                node = Node::new_node(NodeKind::ND_SUB, node, Node::mul(token));
            }else {
                return node;
            }
        }
    }

    pub fn mul(token: &mut VecDeque<Token>) -> Box<Node> {
        let mut node = Node::primary(token);

        loop {
            if consume(token, TokenKind::MUL) {
                node = Node::new_node(NodeKind::ND_MUL, node, Node::primary(token));
            }else if consume(token, TokenKind::DIV) {
                node = Node::new_node(NodeKind::ND_DIV, node, Node::primary(token));
            }else{
                return node;
            }
        }
    }

    pub fn primary(token: &mut VecDeque<Token>) -> Box<Node> {
        if consume(token, TokenKind::LPAR) {
            let node = Node::expr(token);
            expect(token, TokenKind::RPAR);
            return node;
        }else {
            return Node::new_node_num(expect_number(token));
        }
    }
}

pub fn gen(node: Box<Node>) {
    match *node {
        Node::Nil => { //これが検出されたらただのバグ
            eprintln!("Nil pointerです");
            std::process::exit(1);
        }
        Node::Elm { kind, lhs, rhs, val } => {
            if kind == NodeKind::ND_NUM { //トークンが数字の時
                match val{
                    Some(x) => {
                        println!("  push {}", x);
                        return;
                    }
                    None => {
                        eprintln!("valがNoneになってます");
                        std::process::exit(1);
                    }
                }
            }

            gen(lhs);
            gen(rhs);

            println!("  pop rdi");
            println!("  pop rax");

            match kind {
                NodeKind::ND_ADD => println!("  add rax, rdi"),
                NodeKind::ND_SUB => println!("  sub rax, rdi"),
                NodeKind::ND_MUL => println!("  imul rax, rdi"),
                NodeKind::ND_DIV => {
                    println!("  cqo");
                    println!("  idiv rdi");
                }
                _ => (), //それ以外のケースでは何もしない
            }

            println!("  push rax");
        }
    }
} 

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    ADD,    //足し算の記号
    SUB,    //引き算の記号
    MUL,    //掛け算の記号
    DIV,    //割り算の記号
    TKNUM,  // 整数トークン
    LPAR,   //開きかっこ(
    RPAR,   //閉じかっこ)
    TKEOF,  // 入力の終わりを表すトークン
}

#[derive(Debug)]
pub struct Token {
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
    let node = Node::expr(&mut token);

    // println!("{:?}", node);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    // println!("  mov rax, {}", expect_number(&mut token)); //はじめは数字、それ以外の場合はエラー

    gen(node);
    println!("  pop rax");
    println!("  ret");
}
