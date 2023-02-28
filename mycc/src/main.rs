use std::env;
use mycc::{
    token,
    parser::CodeGen,
    syntax_tree::GenerativeRule,
};

fn main() {
    let mut argv: Vec<String> = env::args().collect(); //コマンドライン引数を受け取る
    let argc = argv.len();

    if argc != 2 {
        eprintln!("Incorrect number of arguments. "); //標準エラー出力
        std::process::exit(1); 
    }

    let mut token = token::Tokenize::tokenize(&mut argv[1]); //コマンドラインで受け取った文字列をトークン列に変換する, ここまでOK
    // println!("{:?}", token);
    let prog = GenerativeRule::program(&mut token);
    // println!("{:?}", node);

    let mut codegen = CodeGen::new();
    codegen.codegen(prog);
}
