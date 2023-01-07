use crate::node::{Node, NodeKind};

//構文木からアセンブリ言語を生成する
//もう少し綺麗に書く方法ありそうだけど...
pub fn gen(node: Box<Node>) {
    match *node {
        Node::Nil => { //これが検出されたらただのバグ
            eprintln!("Nil pointerです");
            std::process::exit(1);
        }
        Node::Elm { kind, lhs, rhs, val } => {
            if kind == NodeKind::NDNUM { //トークンが数字の時
                if let Some(x) = val { //Noneチェック
                    println!("  push {}", x);
                    return;
                }else{
                    eprintln!("valがNoneになってます");
                    std::process::exit(1);
                }
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