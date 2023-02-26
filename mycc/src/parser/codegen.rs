use crate::{parser::CodeGen, syntax_tree::Function};

impl CodeGen {
    // コンストラクタ
    pub fn new() -> CodeGen {
        CodeGen {
            label_count: 1
        }
    }

    // プログラムを生成する関数
    // これだけpublicに設定する（たぶん）
    pub fn codegen(&mut self, prog: Function) {
        println!(".intel_syntax noprefix");
        println!(".globl main");
        println!("main:");

        // 後で変更する
        // プロローグ
        // 変数26個分の領域を確保する
        println!("  push rbp");
        println!("  mov rbp, rsp");
        println!("  sub rsp, 208");

        //一行ずつ実行していく
        //ここは後で変更される気がする
        let mut node = prog.body;
        while !node.is_empty() {
            let elem_node = node.pop_front();
            if let Some(n) = elem_node {
                CodeGen::gen(self, n);
            }else{
                eprintln!("コードが不正です");
                std::process::exit(1)
            }
        }

        // node::stack_assembly::gen(node); //アセンブリ言語を出力
        println!("  pop rax");
        // println!(".L.return:");
        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
    }
}