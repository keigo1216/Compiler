use std::env;
use mycc::{
    token,
    node,
};

fn main() {
    let mut argv: Vec<String> = env::args().collect(); //コマンドライン引数を受け取る
    let argc = argv.len();

    if argc != 2 {
        eprintln!("引数の数が正しくありません。"); //標準エラー出力
        std::process::exit(1); 
    }

    let mut token = token::tokenize::tokenize(&mut argv[1]); //コマンドラインで受け取った文字列をトークン列に変換する
    let node = node::util::expr(&mut token);

    // println!("{:?}", node);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    // println!("  mov rax, {}", expect_number(&mut token)); //はじめは数字、それ以外の場合はエラー

    node::stack_assembly::gen(node);
    println!("  pop rax");
    println!("  ret");
}
