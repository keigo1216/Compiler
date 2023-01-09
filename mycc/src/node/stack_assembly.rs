use crate::node::{Node, NodeKind};

pub fn gen_lval(kind: NodeKind, offset: Option<i32>) { //参照で渡さないと所有権がこっちにきてしまうみたい
    // if kind != NodeKind::NDLVAR {
    //     eprintln!("代入の左辺値が数字ではありません");
    //     std::process::exit(1);
    // }

    if let NodeKind::NDLVAR = kind { //トークンがローカル変数の時
        println!("  mov rax, rbp");
        if let Some(x) = offset {
            println!("  sub rax, {}", x);
        }else{ 
            eprintln!("数値が無効です");
            std::process::exit(1);
        }
        println!("  push rax");
    }else{ //トークンがローカル変数ではない時
        eprintln!("代入の左辺値が数字ではありません");
        std::process::exit(1);
    }
}

//構文木からアセンブリ言語を生成する
//もう少し綺麗に書く方法ありそうだけど...
pub fn gen(node: Box<Node>) {
    match *node { //参照外し
        Node::Nil => { //これが検出されたらただのバグ
            eprintln!("Nil pointerです");
            std::process::exit(1);
        }
        Node::Elm { kind, lhs, rhs, val , offset} => { //所有権移っている？
            if kind == NodeKind::NDNUM { //トークンが数字の時
                if let Some(x) = val { //Noneチェック
                    println!("  push {}", x);
                    return;
                }else{
                    eprintln!("valがNoneになってます");
                    std::process::exit(1);
                }
            }

            if kind == NodeKind::NDLVAR {
                gen_lval(kind, offset);
                println!("  pop rax");
                println!("  mov rax, [rax]");
                println!("  push rax");
                return;
            }

            if kind == NodeKind::NDASS {
                match *lhs {
                    Node::Nil => {
                        eprintln!("代入の左辺値が変数ではありません");
                    }
                    Node::Elm { kind, lhs, rhs, val, offset } => {
                        gen_lval(kind, offset);
                    }
                }
                // gen_lval(kind, offset);
                gen(rhs);
                println!("  pop rdi");
                println!("  pop rax");
                println!("  mov [rax], rdi");
                println!("  push rdi");
                return;
            }

            gen(lhs);
            gen(rhs);

            println!("  pop rdi");
            println!("  pop rax");

            match kind {
                NodeKind::NDEQ => {
                    println!("  cmp rax, rdi");
                    println!("  sete al");
                    println!("  movzb rax, al");
                },
                NodeKind::NDNEQ => {
                    println!("  cmp rax, rdi");
                    println!("  setne al");
                    println!("  movzb rax, al");
                }
                NodeKind::NDLT => {
                    println!("  cmp rax, rdi");
                    println!("  setl al");
                    println!("  movzb rax, al");
                },
                NodeKind::NDLE => {
                    println!("  cmp rax, rdi");
                    println!("  setle al");
                    println!("  movzb rax, al");
                },
                NodeKind::NDGT => {
                    println!("  xchg rdi, rax");
                    println!("  cmp rax, rdi");
                    println!("  setl al");
                    println!("  movzb rax, al");
                },
                NodeKind::NDGE => {
                    println!("  xchg rdi, rax");
                    println!("  cmp rax, rdi");
                    println!("  setle al");
                    println!("  movzb rax, al");
                },
                NodeKind::NDADD => println!("  add rax, rdi"),
                NodeKind::NDSUB => println!("  sub rax, rdi"),
                NodeKind::NDMUL => println!("  imul rax, rdi"),
                NodeKind::NDDIV => {
                    println!("  cqo");
                    println!("  idiv rdi");
                }
                _ => (), //それ以外のケースでは何もしない
            }

            println!("  push rax");
        }
    }
} 