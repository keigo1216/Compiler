use crate::{parser::CodeGen, syntax_tree::Function};

impl CodeGen {
    // コンストラクタ
    pub fn new() -> CodeGen {
        CodeGen {
            label_count: 1
        }
    }

    pub fn align_to(n: i32, align: i32) -> i32 {
        return (n + align - 1) / align * align;
    }

    pub(super) fn assign_lvar_offsets(prog: &mut Function) {
        let offset = prog.locals.len() as i32;
        prog.stack_size = CodeGen::align_to(offset, 16);
    }

    // プログラムを生成する関数
    // これだけpublicに設定する（たぶん）
    pub fn codegen(&mut self, mut prog: Function) {
        CodeGen::assign_lvar_offsets(&mut prog);

        println!(".intel_syntax noprefix");
        println!(".globl main");
        println!("main:");

        // 後で変更する
        // プロローグ
        // 変数26個分の領域を確保する
        println!("  push rbp");
        println!("  mov rbp, rsp");
        println!("  sub rsp, {}", prog.stack_size);

        //一行ずつ実行していく
        //ここは後で変更される気がする
        let mut node = prog.body;
        while !node.is_empty() {
            let elem_node = node.pop_front();
            if let Some(n) = elem_node {
                CodeGen::gen_stmt(self, n);
            }else{
                eprintln!("コードが不正です");
                std::process::exit(1)
            }
        }

        // node::stack_assembly::gen(node); //アセンブリ言語を出力
        // println!("  pop rax");
        println!(".L.return:");
        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
    }
}