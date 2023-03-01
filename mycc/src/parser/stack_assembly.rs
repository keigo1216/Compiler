use crate::syntax_tree::{Node, NodeKind};
use crate::parser::CodeGen;

impl CodeGen {
    pub(super) fn push() {
        println!("  push rax");
    }

    pub(super) fn pop(reg: String) {
        println!("  pop {}", reg);
    }

    // ジャンプの時に使うラベル
    pub(super) fn count(&mut self) -> i32 {
        self.label_count += 1;
        return self.label_count
    }

    //ベースポインタから_offsetの位置のアドレスをraxに格納
    pub(super) fn gen_addr(_offset: i32) {
        println!("  mov rax, rbp");
        println!("  sub rax, {}", _offset);
    } 

    // gen_addrのラッパー, 引数がBox<Node>
    pub(super) fn gen_addr_node(node: Box<Node>) {
        if let Node::Elm { kind: NodeKind::NDLVAR, offset: Some(_offset), ..} = *node {
            CodeGen::gen_addr(_offset);
        } else {
            eprintln!("gen_addr_node: this node is not NDLVAR");
            std::process::exit(1);
        }
    }

    //戻り値なし
    //計算結果は全てraxレジスタに格納
    pub(super) fn gen_expr(&mut self, node: Box<Node>) {
        match *node {
            Node::Nil => {
                eprint!("gen_expr error: this node is Nil");
                std::process::exit(1);
            }
            Node::Elm { kind: NodeKind::NDNUM, val: Some(_val), ..} => { // 定数
                println!("  mov rax, {}", _val) // 直値いけるのか問題
            }
            Node::Elm { kind: NodeKind::NDLVAR, offset: Some(_offset), ..} => { // 変数
                CodeGen::gen_addr(_offset);
                println!("  mov rax, [rax]");
            }
            Node::Elm { kind: NodeKind::NDASS, lhs, rhs, ..} => { // 代入式
                /*
                rax: 右辺の値
                rdi: 左辺の変数が格納されているアドレス
                 */
                CodeGen::gen_addr_node(lhs);
                CodeGen::push();
                CodeGen::gen_expr(self, rhs);
                CodeGen::pop("rdi".to_string());
                println!("  mov [rdi], rax");
            }
            Node::Elm { kind, lhs, rhs, ..} => { // それ以外
                /*
                gen_exprを実行すると, raxに計算結果が格納されている
                まず, 右部分木の計算を実行し, raxに入っている計算結果をpushする
                その後, 左部分木の計算を実行し, raxに格納される
                最後にpopすることで, rdiに右部分木の計算結果が格納される
                 */
                CodeGen::gen_expr(self, rhs);
                CodeGen::push();
                CodeGen::gen_expr(self, lhs);
                CodeGen::pop("rdi".to_string());

                match kind {
                    NodeKind::NDADD => {
                        println!("  add rax, rdi");
                    }
                    NodeKind::NDSUB => {
                        println!("  sub rax, rdi");
                    }
                    NodeKind::NDMUL => {
                        println!("  imul rax, rdi");
                    }
                    NodeKind::NDDIV => {
                        println!("  cqo");
                        println!("  idiv rdi");
                    }
                    NodeKind::NDEQ => {
                        println!("  cmp rax, rdi");
                        println!("  sete al");
                        println!("  movzb rax, al");
                    }
                    NodeKind::NDNEQ => {
                        println!("  cmp rax, rdi");
                        println!("  setne al");
                        println!("  movzb rax, al");
                    }
                    NodeKind::NDLT => {
                        println!("  cmp rax, rdi");
                        println!("  setl al");
                        println!("  movzb rax, al");
                    }
                    NodeKind::NDLE => {
                        println!("  cmp rax, rdi");
                        println!("  setle al");
                        println!("  movzb rax, al");
                    }
                    _ => {
                        eprintln!("gen_expr: not match");
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    pub(super) fn gen_stmt(&mut self, node: Box<Node>) {
        match *node {
            Node::Nil => {
                eprint!("gen_stmt error: this node is Nil");
                std::process::exit(1);
            }
            Node::Elm { kind: NodeKind::NDBLOCK, mut body , ..} => { // block
                while !body.is_empty() {
                    let elem_node = body.pop_front();
                    if let Some(n) = elem_node {
                        CodeGen::gen_stmt(self, n);
                    } else {
                        eprintln!("コードが不正です");
                        std::process::exit(1);
                    }
                }
            }
            Node::Elm { kind: NodeKind::NDIF, cond, then, els, ..} => { // if
                let _label_count = CodeGen::count(self);
                CodeGen::gen_expr(self, cond);
                println!("  cmp rax, 0");
                println!("  je .L.else.{}", _label_count);
                CodeGen::gen_stmt(self, then);
                println!("  jmp .L.end.{}", _label_count);
                println!(".L.else.{}:", _label_count);
                if *els != Node::Nil {
                    CodeGen::gen_stmt(self, els);
                }
                println!(".L.end.{}:", _label_count);
            }
            Node::Elm { kind: NodeKind::NDFOR, cond, then, init, inc, ..} => { // for
                let _label_count = CodeGen::count(self);
                if *init != Node::Nil {
                    CodeGen::gen_stmt(self, init);
                }
                println!(".L.begin.{}:", _label_count);
                if *cond != Node::Nil {
                    CodeGen::gen_expr(self, cond);
                    println!("  cmp rax, 0");
                    println!("  je  .L.end.{}", _label_count);
                }
                CodeGen::gen_stmt(self, then);
                if *inc != Node::Nil {
                    CodeGen::gen_expr(self, inc);
                }
                println!("  jmp .L.begin.{}", _label_count);
                println!(".L.end.{}:", _label_count);
                return;
            }
            Node::Elm { kind: NodeKind::NDRETURN, lhs, ..} => { // return
                CodeGen::gen_stmt(self, lhs);
                println!("  jmp .L.return");
            }
            _ => {
                // eprint!("gen_stmt error: this node is not BLOCK ot RETURN, invalid node");
                // std::process::exit(1);
                CodeGen::gen_expr(self, node);
            }
        }
    }
}