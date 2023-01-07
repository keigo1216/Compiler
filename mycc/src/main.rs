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

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    node::stack_assembly::gen(node); //アセンブリ言語を出力

    println!("  pop rax");
    println!("  ret");
}
