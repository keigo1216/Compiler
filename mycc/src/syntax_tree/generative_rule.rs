use std::collections::VecDeque;
use crate::token::{Token, TokenKind};
use crate::syntax_tree::{Obj, Node, NodeKind, GenerativeRule};

use super::Function;

impl GenerativeRule {
    //非終端記号
    //生成規則を書いてます

    //一つのプログラムの塊は複数のステートメントで成り立っている
    //生成規則
    //program = stmt*
    // *は一つ以上を表す正規表現
    pub fn program(token: &mut VecDeque<Token>) -> Function {
        let mut code: VecDeque<Box<Node>> = VecDeque::new(); //構文木を入れるキュー
        let mut vec_obj: Vec<Obj> = Vec::new(); //ローカル変数の種類とその位置を格納する
        while !GenerativeRule::at_eof(token) {
            code.push_back(GenerativeRule::stmt(token, &mut vec_obj));
            // GenerativeRule::compound_stmt(token, &mut vec_obj);x
        }
        Function::new(code, vec_obj)
        // code
    }

    //一つのステートメントの一番最後はセミコロンで書かれている
    //生成規則
    /*
    stmt = expr ';' 
        | "{" stmt* "}"
        | "return" expr ";"
        | "if" "(" expr ")" stmt ("else" stmt) ?
    */
    //stmt = expr ';' | "return" expr ";"
    pub fn stmt(token: &mut VecDeque<Token>, vec_obj: &mut Vec<Obj>) -> Box<Node> {
        if GenerativeRule::consume(token, TokenKind::RETURN) { // "return" expr ";"
            let node = Node::new_node(NodeKind::NDRETURN, GenerativeRule::expr(token, vec_obj), Box::new(Node::Nil));
            GenerativeRule::expect(token, TokenKind::SEMI);
            return node;
        } else if GenerativeRule::consume(token, TokenKind::IF) { //if文の時
            GenerativeRule::expect(token, TokenKind::LPAR); // "("がないとエラー
            let cond = GenerativeRule::expr(token, vec_obj); //ifの条件文
            GenerativeRule::expect(token, TokenKind::RPAR); // ")"がないとエラー
            let then = GenerativeRule::stmt(token, vec_obj); //ifの中の式
            if GenerativeRule::consume(token, TokenKind::ELS) {
                let els = GenerativeRule::stmt(token, vec_obj);
                return Node::new_node_if(cond, then, els);
            } else {
                return Node::new_node_if(cond, then, Box::new(Node::Nil));
            }
        } else if GenerativeRule::consume(token, TokenKind::LBLOCK) { // "{" stmt* "}"
            let mut code_block: VecDeque<Box<Node>> = VecDeque::new();
            while !GenerativeRule::consume(token, TokenKind::RBLOCK) { // "}"トークンが来るまで
                code_block.push_back(GenerativeRule::stmt(token, vec_obj));
            }
            return Node::new_node_block(code_block);
        } else {
            let node = GenerativeRule::expr(token, vec_obj);
            GenerativeRule::expect(token, TokenKind::SEMI);
            return node;
        }
        // util::expect(token, TokenKind::SEMI); //最後がセミコロンでない時はエラーを出す
        // node
    }

    //これの生成規則が何を意味するのかがあんまりわからない
    //生成規則
    //expr = assign
    pub fn expr(token: &mut VecDeque<Token>, vec_obj: &mut Vec<Obj>) -> Box<Node> {
        // equality(token)
        GenerativeRule::assign(token, vec_obj)
    }

    //assignは代入文、または通常の式を表現する
    //生成規則
    //assign = equality ("=" assign)?
    pub fn assign(token: &mut VecDeque<Token>, vec_obj: &mut Vec<Obj>) -> Box<Node> {
        let mut node = GenerativeRule::equality(token, vec_obj);

        loop {
            if GenerativeRule::consume(token, TokenKind::ASS) { //= 代入文の時
                node = Node::new_node(NodeKind::NDASS, node, GenerativeRule::assign(token, vec_obj));
            }else{
                return node;
            }
        }
    }

    //==とノットイコールを生成する
    //生成規則
    //equality   = relational ("==" relational | "!=" relational)*
    pub fn equality(token: &mut VecDeque<Token>, vec_obj: &mut Vec<Obj>) -> Box<Node> {
        let mut node = GenerativeRule::relational(token, vec_obj);

        loop {
            if GenerativeRule::consume(token, TokenKind::EQ) { //==
                node = Node::new_node(NodeKind::NDEQ, node, GenerativeRule::relational(token, vec_obj));
            }else if GenerativeRule::consume(token, TokenKind::NEQ) { //ノットイコール
                node = Node::new_node(NodeKind::NDNEQ, node, GenerativeRule::relational(token, vec_obj));
            }else{
                return node;
            }
        }
    }

    //比較演算子を生成する
    //生成規則
    //relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    pub fn relational(token: &mut VecDeque<Token>, vec_obj: &mut Vec<Obj>) -> Box<Node> {
        let mut node = GenerativeRule::add(token, vec_obj);

        loop {
            if GenerativeRule::consume(token, TokenKind::LT) {
                node = Node::new_node(NodeKind::NDLT, node, GenerativeRule::add(token, vec_obj));
            }else if GenerativeRule::consume(token, TokenKind::LE) {
                node = Node::new_node(NodeKind::NDLE, node, GenerativeRule::add(token, vec_obj));
            }else if GenerativeRule::consume(token, TokenKind::GT) {
                node = Node::new_node(NodeKind::NDGT, node, GenerativeRule::add(token, vec_obj));
            }else if GenerativeRule::consume(token, TokenKind::GE) {
                node = Node::new_node(NodeKind::NDGE, node, GenerativeRule::add(token, vec_obj));
            }else{
                return node;
            }
        }
    }

    //足し算、引き算の演算子を生成する
    //生成規則
    //add = mul ("+" mul | "-" mul)*
    pub fn add (token: &mut VecDeque<Token>, vec_obj: &mut Vec<Obj>) -> Box<Node> {
        let mut node = GenerativeRule::mul(token, vec_obj);

        loop {
            if GenerativeRule::consume(token, TokenKind::ADD) {
                node = Node::new_node(NodeKind::NDADD, node, GenerativeRule::mul(token, vec_obj));
            }else if GenerativeRule::consume(token, TokenKind::SUB) {
                node = Node::new_node(NodeKind::NDSUB, node, GenerativeRule::mul(token, vec_obj));
            }else{
                return node;
            }
        }
    }

    //掛け算、割り算の演算子を生成する
    //生成規則
    //mul = unary ("*" unary | "/" unary)*
    pub fn mul(token: &mut VecDeque<Token>, vec_obj: &mut Vec<Obj>) -> Box<Node> {
        let mut node = GenerativeRule::unary(token, vec_obj);

        loop {
            if GenerativeRule::consume(token, TokenKind::MUL) {
                node = Node::new_node(NodeKind::NDMUL, node, GenerativeRule::unary(token, vec_obj));
            }else if GenerativeRule::consume(token, TokenKind::DIV) {
                node = Node::new_node(NodeKind::NDDIV, node, GenerativeRule::unary(token, vec_obj));
            }else{
                return node;
            }
        }
    }

    //単項演算子を生成する
    //数字の±のところ
    //たぶんここの生成規則間違っている
    //生成規則
    //unary = ("+" | "-")? primary（たぶんこれ間違ってる）
    //unary = ("+" | "-") unary | primary
    //?は0か1つ
    pub fn unary(token: &mut VecDeque<Token>, vec_obj: &mut Vec<Obj>) -> Box<Node> {
        if GenerativeRule::consume(token, TokenKind::ADD) {
            GenerativeRule::unary(token, vec_obj)
        }else if GenerativeRule::consume(token, TokenKind::SUB) {
            Node::new_node(NodeKind::NDSUB, Node::new_node_num(0), GenerativeRule::unary(token, vec_obj))
        }else{
            GenerativeRule::primary(token, vec_obj)
        }
    }


    //数字、識別子（変数名）、カッコを生成する
    //生成規則
    //primary = num | ident | "(" expr ")"
    pub fn primary(token: &mut VecDeque<Token>, vec_obj: &mut Vec<Obj>) -> Box<Node> {
        if GenerativeRule::consume(token, TokenKind::LPAR) { //(の時
            let node = GenerativeRule::expr(token, vec_obj);
            GenerativeRule::expect(token, TokenKind::RPAR);
            return node;
        }else if GenerativeRule::consume_ident(token) { //IDの時, ここ汚すぎですね、後で変更するはず
            //新しい変数かどうかをチェック
            let s = GenerativeRule::expect_id(token); //トークンの文字列を取得
            let result_find = (&vec_obj).iter().find(|v| &v.name == &s); //vec_objにあるかどうか

            match result_find {
                Some(obj) => { //変数が存在した場合
                    return Node::new_node_var(obj.offset)
                },
                None => { //変数が存在しなかった場合
                    let offset = ((vec_obj.len() + 1) * 8) as i32; //現状の変数の数からベースポインタへのオフセットを計算する

                    let obj_to_push = Obj {
                        name: s.clone(),
                        offset
                    };
                    vec_obj.push(obj_to_push); //変数を追加

                    return Node::new_node_var(offset);
                }
            }
        }   
        
        else {
            return Node::new_node_num(GenerativeRule::expect_number(token)); //数字があれば数字を返す、なければエラーを出す
        }
    }
}