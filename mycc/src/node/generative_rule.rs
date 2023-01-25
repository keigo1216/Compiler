use std::collections::VecDeque;
use crate::token::{Token, TokenKind};
use crate::node::{Node, NodeKind, util};

pub struct LVar {
    name: String, //ローカル変数
    offset: i32, //ベースポインタからのオフセット
}

//非終端記号
//生成規則を書いてます

//一つのプログラムの塊は複数のステートメントで成り立っている
//生成規則
//program = stmt*
// *は一つ以上を表す正規表現
pub fn program(token: &mut VecDeque<Token>) -> VecDeque<Box<Node>> {
    let mut code: VecDeque<Box<Node>> = VecDeque::new(); //構文木を入れるキュー
    let mut vec_lvar: Vec<LVar> = Vec::new(); //ローカル変数の種類とその位置を格納する
    while !util::at_eof(token) {
        code.push_back(stmt(token, &mut vec_lvar));
    }
    code
}

//一つのステートメントの一番最後はセミコロンで書かれている
//生成規則
//stmt = expr ';' | "return" expr ";"
pub fn stmt(token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
    if util::consume(token, TokenKind::RETURN) {
        let node = Node::new_node(NodeKind::NDRETURN, expr(token, vec_lvar), Box::new(Node::Nil));
        util::expect(token, TokenKind::SEMI);
        return node;
    } else {
        let node = expr(token, vec_lvar);
        util::expect(token, TokenKind::SEMI);
        return node;
    }
    // util::expect(token, TokenKind::SEMI); //最後がセミコロンでない時はエラーを出す
    // node
}

//これの生成規則が何を意味するのかがあんまりわからない
//生成規則
//expr = assign
pub fn expr(token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
    // equality(token)
    assign(token, vec_lvar)
}

//assignは代入文、または通常の式を表現する
//生成規則
//assign = equality ("=" assign)?
pub fn assign(token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
    let mut node = equality(token, vec_lvar);

    loop {
        if util::consume(token, TokenKind::ASS) { //= 代入文の時
            node = Node::new_node(NodeKind::NDASS, node, assign(token, vec_lvar));
        }else{
            return node;
        }
    }
}

//==とノットイコールを生成する
//生成規則
//equality   = relational ("==" relational | "!=" relational)*
pub fn equality(token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
    let mut node = relational(token, vec_lvar);

    loop {
        if util::consume(token, TokenKind::EQ) { //==
            node = Node::new_node(NodeKind::NDEQ, node, relational(token, vec_lvar));
        }else if util::consume(token, TokenKind::NEQ) { //ノットイコール
            node = Node::new_node(NodeKind::NDNEQ, node, relational(token, vec_lvar));
        }else{
            return node;
        }
    }
}

//比較演算子を生成する
//生成規則
//relational = add ("<" add | "<=" add | ">" add | ">=" add)*
pub fn relational(token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
    let mut node = add(token, vec_lvar);

    loop {
        if util::consume(token, TokenKind::LT) {
            node = Node::new_node(NodeKind::NDLT, node, add(token, vec_lvar));
        }else if util::consume(token, TokenKind::LE) {
            node = Node::new_node(NodeKind::NDLE, node, add(token, vec_lvar));
        }else if util::consume(token, TokenKind::GT) {
            node = Node::new_node(NodeKind::NDGT, node, add(token, vec_lvar));
        }else if util::consume(token, TokenKind::GE) {
            node = Node::new_node(NodeKind::NDGE, node, add(token, vec_lvar));
        }else{
            return node;
        }
    }
}

//足し算、引き算の演算子を生成する
//生成規則
//add = mul ("+" mul | "-" mul)*
pub fn add (token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
    let mut node = mul(token, vec_lvar);

    loop {
        if util::consume(token, TokenKind::ADD) {
            node = Node::new_node(NodeKind::NDADD, node, mul(token, vec_lvar));
        }else if util::consume(token, TokenKind::SUB) {
            node = Node::new_node(NodeKind::NDSUB, node, mul(token, vec_lvar));
        }else{
            return node;
        }
    }
}

//掛け算、割り算の演算子を生成する
//生成規則
//mul = unary ("*" unary | "/" unary)*
pub fn mul(token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
    let mut node = unary(token, vec_lvar);

    loop {
        if util::consume(token, TokenKind::MUL) {
            node = Node::new_node(NodeKind::NDMUL, node, unary(token, vec_lvar));
        }else if util::consume(token, TokenKind::DIV) {
            node = Node::new_node(NodeKind::NDDIV, node, unary(token, vec_lvar));
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
pub fn unary(token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
    if util::consume(token, TokenKind::ADD) {
        unary(token, vec_lvar)
    }else if util::consume(token, TokenKind::SUB) {
        Node::new_node(NodeKind::NDSUB, Node::new_node_num(0), unary(token, vec_lvar))
    }else{
        primary(token, vec_lvar)
    }
}


//数字、識別子（変数名）、カッコを生成する
//生成規則
//primary = num | ident | "(" expr ")"
pub fn primary(token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
    if util::consume(token, TokenKind::LPAR) { //(の時
        let node = expr(token, vec_lvar);
        util::expect(token, TokenKind::RPAR);
        return node;
    }else if util::consume_ident(token) { //IDの時, ここ汚すぎですね、後で変更するはず
        //新しい変数かどうかをチェック
        let s = util::expect_id(token); //トークンの文字列を取得
        let result_find = (&vec_lvar).iter().find(|v| &v.name == &s); //vec_lvarにあるかどうか

        match result_find {
            Some(lvar) => { //変数が存在した場合
                return Node::new_node_var(lvar.offset)
            },
            None => { //変数が存在しなかった場合
                let num_lvar = ((vec_lvar.len() + 1) * 8) as i32; //現状の変数の数からベースポインタへのオフセットを計算する

                let lvar_to_push = LVar {
                    name: s.clone(),
                    offset: num_lvar,
                };
                vec_lvar.push(lvar_to_push); //変数を追加

                return Node::new_node_var(num_lvar);
            }
        }
    }   
    
    else {
        return Node::new_node_num(util::expect_number(token)); //数字があれば数字を返す、なければエラーを出す
    }
}