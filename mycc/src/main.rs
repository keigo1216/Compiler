use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect(); //コマンドライン引数を受け取る
    let argc = argv.len();

    if argc != 2 {
        eprintln!("引数の数が正しくありません。"); //標準エラー出力
        std::process::exit(1); 
    }

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", argv[1]);
    println!("  ret");
}
