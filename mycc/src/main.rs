use std::env;

//与えられた文字列リテラルを+と-区切りで分ける
fn split_string(s: &str) -> Vec<&str> {
    let mut v: Vec<&str> = Vec::new();

    let mut begin = 0;
    let mut end = 0;

    for c in s.chars() { //文字列を文字ごとに取得
        if c == '+' {
            v.push(&s[begin..end]);
            v.push("+");
            end += 1;
            begin = end;
        } else if c == '-' {
            v.push(&s[begin..end]);
            v.push("-");
            end += 1;
            begin = end;
        } else {
            end += 1;
        }
    }
    v.push(&s[begin..]); //一番最後を追加するのを忘れずに

    v
}

fn main() {
    let argv: Vec<String> = env::args().collect(); //コマンドライン引数を受け取る
    let argc = argv.len();

    if argc != 2 {
        eprintln!("引数の数が正しくありません。"); //標準エラー出力
        std::process::exit(1); 
    }

    let split_string_vec = split_string(&argv[1][..]); //文字列を+-区切りで表す
    let num_symbol = (split_string_vec.len() - 1) / 2;

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", split_string_vec[0]);

    for i in 0..num_symbol {
        let symbol = split_string_vec[1 + 2*i]; //演算子
        let num = split_string_vec[2*(i+1)]; //数字

        if symbol == "+" {
            println!("  add rax, {}", num);
        }else if symbol == "-" {
            println!("  sub rax, {}", num);
        }else{
            eprintln!("予期しない文字です: {}", symbol); //標準エラー出力
             std::process::exit(1); 
        }
    }

    println!("  ret");
}
