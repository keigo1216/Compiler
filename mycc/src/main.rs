use std::env;
use mycc::{
    token,
    node,
    parser,
};

fn main() {
    let mut argv: Vec<String> = env::args().collect(); //コマンドライン引数を受け取る
    let argc = argv.len();

    if argc != 2 {
        eprintln!("Incorrect number of arguments. "); //標準エラー出力
        std::process::exit(1); 
    }

    let mut token = token::tokenize::tokenize(&mut argv[1]); //コマンドラインで受け取った文字列をトークン列に変換する, ここまでOK
    // println!("{:?}", token);
    let mut node = node::generative_rule::program(&mut token);
    // println!("{:?}", node);

    

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    // プロローグ
    // 変数26個分の領域を確保する
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    //一行ずつ実行していく
    //ここは後で変更される気がする（たぶん）
    while !node.is_empty() {
        let elem_node = node.pop_front();
        if let Some(n) = elem_node {
            parser::stack_assembly::gen(n);
        }else{
            eprintln!("コードが不正です");
            std::process::exit(1)
        }
    }
    // node::stack_assembly::gen(node); //アセンブリ言語を出力

    println!("  pop rax");
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}
