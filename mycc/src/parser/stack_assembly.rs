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


    // //構文木からアセンブリ言語を生成する
    // pub(super) fn gen(&mut self, node: Box<Node>) {
    //     match *node { //参照外し
    //         Node::Nil => { //これが検出されたらただのバグ
    //             eprintln!("this node is Nil, so you have to fix bug in gen or so");
    //             std::process::exit(1);
    //         }
    //         Node::Elm { kind: NodeKind::NDBLOCK, mut body, .. } => { //blockの時
    //             while !body.is_empty() {
    //                 let elem_node = body.pop_front();
    //                 if let Some(n) = elem_node {
    //                     CodeGen::gen(self, n);
    //                 } else {
    //                     eprintln!("コードが不正です");
    //                     std::process::exit(1);
    //                 }
    //             }
    //         }
    //         Node::Elm { kind: NodeKind::NDIF, cond, then, els, .. } => { //if文の時
    //             let _label_count = CodeGen::count(self);
    //             CodeGen::gen(self, cond); //条件文のコードを先に生成
    //             println!("  pop rax");
    //             println!("  cmp rax, 0");
    //             println!("  je .Lelse{}", _label_count);
    //             CodeGen::gen(self, then);
    //             println!("  jmp .Lend{}", _label_count);
    //             println!(".Lelse{}:", _label_count);
    //             if *els != Node::Nil { // else文があったら, ここもう少し綺麗に書けそう
    //                 CodeGen::gen(self, els);
    //             }
    //             println!(".Lend{}:", _label_count);
    //         }
    //         Node::Elm { kind: NodeKind::NDRETURN, lhs ,..} => { //returnの時
    //             CodeGen::gen(self, lhs);
    //             println!("  pop rax");
    //             println!("  mov rsp, rbp");
    //             println!("  pop rbp");
    //             println!("  ret");
    //         }
    //         Node::Elm { kind: NodeKind::NDNUM, val: Some(_val),..} => { //数字のとき
    //             println!("  push {}", _val);
    //         }
    //         Node::Elm { kind:NodeKind::NDLVAR, offset: Some(_offset), ..} => { //変数のとき
    //             CodeGen::gen_lval(_offset); //変数の宣言
    //             println!("  pop rax");
    //             println!("  mov rax, [rax]");
    //             println!("  push rax");
    //         }
    //         Node::Elm { kind:NodeKind::NDASS, lhs, rhs, ..} => { //代入式のとき
    //             if let Node::Elm{ kind: NodeKind::NDLVAR, offset: Some(_offset), ..} = *lhs {
    //                 CodeGen::gen_lval(_offset); //変数の宣言
    //             }else{
    //                 eprintln!("不正なトークンです. 構文解析できません.");
    //                 std::process::exit(1);
    //             }
    //             CodeGen::gen(self, rhs);
    //             println!("  pop rdi");
    //             println!("  pop rax");
    //             println!("  mov [rax], rdi");
    //             println!("  push rdi");
    //         }
    //         Node::Elm { kind, lhs, rhs, ..} => { //それ以外
    //             CodeGen::gen(self, lhs);
    //             CodeGen::gen(self, rhs);
    //             println!("  pop rdi");
    //             println!("  pop rax");

    //             match kind {
    //                 NodeKind::NDEQ => { //等式
    //                     println!("  cmp rax, rdi");
    //                     println!("  sete al");
    //                     println!("  movzb rax, al");
    //                 },
    //                 NodeKind::NDNEQ => { //ノットイコール
    //                     println!("  cmp rax, rdi");
    //                     println!("  setne al");
    //                     println!("  movzb rax, al");
    //                 }
    //                 NodeKind::NDLT => { //小なり
    //                     println!("  cmp rax, rdi");
    //                     println!("  setl al");
    //                     println!("  movzb rax, al");
    //                 },
    //                 NodeKind::NDLE => { //小なりイコール
    //                     println!("  cmp rax, rdi");
    //                     println!("  setle al");
    //                     println!("  movzb rax, al");
    //                 },
    //                 NodeKind::NDADD => println!("  add rax, rdi"), //足し算
    //                 NodeKind::NDSUB => println!("  sub rax, rdi"), //引き算
    //                 NodeKind::NDMUL => println!("  imul rax, rdi"), //掛け算
    //                 NodeKind::NDDIV => { //割り算
    //                     println!("  cqo");
    //                     println!("  idiv rdi");
    //                 }
    //                 _ => { //ここまでなにも引っかからなかったらバグ
    //                     eprintln!("宣言されていないノードが存在します");
    //                     std::process::exit(1);
    //                 }
    //             }
    //             println!("  push rax");   
    //         }
    //     }
    // } 
}