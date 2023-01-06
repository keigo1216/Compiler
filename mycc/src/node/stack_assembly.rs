use crate::node::{Node, NodeKind};

//構文木からアセンブリ言語を生成する
pub fn gen(node: Box<Node>) {
    match *node {
        Node::Nil => { //これが検出されたらただのバグ
            eprintln!("Nil pointerです");
            std::process::exit(1);
        }
        Node::Elm { kind, lhs, rhs, val } => {
            if kind == NodeKind::NDNUM { //トークンが数字の時
                match val{
                    Some(x) => {
                        println!("  push {}", x);
                        return;
                    }
                    None => {
                        eprintln!("valがNoneになってます");
                        std::process::exit(1);
                    }
                }
            }

            gen(lhs);
            gen(rhs);

            println!("  pop rdi");
            println!("  pop rax");

            match kind {
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