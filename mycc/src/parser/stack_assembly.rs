use crate::syntax_tree::{Node, NodeKind};
use crate::parser::CodeGen;

impl CodeGen {
    pub(super) fn count(&mut self) -> i32 {
        self.label_count += 1;
        return self.label_count
    }

    //変数を宣言する関数
    //ベースポインタからoffsetだけ下がった位置に変数を宣言している
    pub(super) fn gen_lval(_offset: i32) {
        println!("  mov rax, rbp");
        println!("  sub rax, {}", _offset);
        println!("  push rax");
    }


    //構文木からアセンブリ言語を生成する
    pub(super) fn gen(&mut self, node: Box<Node>) {
        match *node { //参照外し
            Node::Nil => { //これが検出されたらただのバグ
                eprintln!("this node is Nil, so you have to fix bug in gen or so");
                std::process::exit(1);
            }
            Node::Elm { kind: NodeKind::NDBLOCK, mut body, .. } => { //blockの時
                while !body.is_empty() {
                    let elem_node = body.pop_front();
                    if let Some(n) = elem_node {
                        CodeGen::gen(self, n);
                    } else {
                        eprintln!("コードが不正です");
                        std::process::exit(1);
                    }
                }
            }
            Node::Elm { kind: NodeKind::NDIF, cond, then, els, .. } => { //if文の時
                let _label_count = CodeGen::count(self);
                CodeGen::gen(self, cond); //条件文のコードを先に生成
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je .Lelse{}", _label_count);
                CodeGen::gen(self, then);
                println!("  jmp .Lend{}", _label_count);
                println!(".Lelse{}:", _label_count);
                if *els != Node::Nil { // else文があったら, ここもう少し綺麗に書けそう
                    CodeGen::gen(self, els);
                }
                println!(".Lend{}:", _label_count);
            }
            Node::Elm { kind: NodeKind::NDRETURN, lhs ,..} => { //returnの時
                CodeGen::gen(self, lhs);
                println!("  pop rax");
                println!("  mov rsp, rbp");
                println!("  pop rbp");
                println!("  ret");
            }
            Node::Elm { kind: NodeKind::NDNUM, val: Some(_val),..} => { //数字のとき
                println!("  push {}", _val);
            }
            Node::Elm { kind:NodeKind::NDLVAR, offset: Some(_offset), ..} => { //変数のとき
                CodeGen::gen_lval(_offset); //変数の宣言
                println!("  pop rax");
                println!("  mov rax, [rax]");
                println!("  push rax");
            }
            Node::Elm { kind:NodeKind::NDASS, lhs, rhs, ..} => { //代入式のとき
                if let Node::Elm{ kind: NodeKind::NDLVAR, offset: Some(_offset), ..} = *lhs {
                    CodeGen::gen_lval(_offset); //変数の宣言
                }else{
                    eprintln!("不正なトークンです. 構文解析できません.");
                    std::process::exit(1);
                }
                CodeGen::gen(self, rhs);
                println!("  pop rdi");
                println!("  pop rax");
                println!("  mov [rax], rdi");
                println!("  push rdi");
            }
            Node::Elm { kind, lhs, rhs, ..} => { //それ以外
                CodeGen::gen(self, lhs);
                CodeGen::gen(self, rhs);
                println!("  pop rdi");
                println!("  pop rax");

                match kind {
                    NodeKind::NDEQ => { //等式
                        println!("  cmp rax, rdi");
                        println!("  sete al");
                        println!("  movzb rax, al");
                    },
                    NodeKind::NDNEQ => { //ノットイコール
                        println!("  cmp rax, rdi");
                        println!("  setne al");
                        println!("  movzb rax, al");
                    }
                    NodeKind::NDLT => { //小なり
                        println!("  cmp rax, rdi");
                        println!("  setl al");
                        println!("  movzb rax, al");
                    },
                    NodeKind::NDLE => { //小なりイコール
                        println!("  cmp rax, rdi");
                        println!("  setle al");
                        println!("  movzb rax, al");
                    },
                    NodeKind::NDGT => { //大なり
                        println!("  xchg rdi, rax");
                        println!("  cmp rax, rdi");
                        println!("  setl al");
                        println!("  movzb rax, al");
                    },
                    NodeKind::NDGE => { //大なりイコール
                        println!("  xchg rdi, rax");
                        println!("  cmp rax, rdi");
                        println!("  setle al");
                        println!("  movzb rax, al");
                    },
                    NodeKind::NDADD => println!("  add rax, rdi"), //足し算
                    NodeKind::NDSUB => println!("  sub rax, rdi"), //引き算
                    NodeKind::NDMUL => println!("  imul rax, rdi"), //掛け算
                    NodeKind::NDDIV => { //割り算
                        println!("  cqo");
                        println!("  idiv rdi");
                    }
                    _ => { //ここまでなにも引っかからなかったらバグ
                        eprintln!("宣言されていないノードが存在します");
                        std::process::exit(1);
                    }
                }
                println!("  push rax");   
            }
        }
    } 
}