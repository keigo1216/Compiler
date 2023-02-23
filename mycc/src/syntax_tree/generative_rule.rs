use std::collections::VecDeque;
use crate::token::{Token, TokenKind};
use crate::syntax_tree::{SyntaxTree, LVar, Node, NodeKind};

impl SyntaxTree {

    //コンストラクタ
    pub fn new(token: VecDeque<Token>) -> SyntaxTree{
        SyntaxTree{
            vec_lvar: Vec::new(),
            token,
            code: VecDeque::new(),
        }
    }

    //vec_varの長さを取得するラッパー
    pub fn get_vec_var_len(&self) -> usize {
        return self.vec_lvar.len();
    }

    //vec_varに要素をpushするラッパー
    pub fn push_vec_var(&mut self, lvar: LVar) {
        self.vec_lvar.push(lvar);
    }

    //生成規則
    /*
    プログラムは複数の文から成り立っている
    program = stmt*
     */
    pub fn program(&mut self) -> VecDeque<Box<Node>> {
        while !self.at_eof() {
            let push_to_code = self.stmt();
            self.code.push_back(push_to_code);
        }
        self.code.clone()
    }

    //一つのステートメントの一番最後はセミコロンで書かれている
    //生成規則
    /*
    stmt = expr ';' 
        | "return" expr ";"
        | "if" "(" expr ")" stmt ("else" stmt) ?
    */
    pub fn stmt(&mut self) -> Box<Node> {
        if self.consume(TokenKind::RETURN) {
            let node = Node::new_node(NodeKind::NDRETURN, self.expr(), Box::new(Node::Nil));
            self.expect(TokenKind::SEMI);
            return node;
        } else if self.consume(TokenKind::IF) { //if文の時
            self.expect(TokenKind::LPAR); // "("がないとエラー
            let cond = self.expr(); //ifの条件文
            self.expect(TokenKind::RPAR); // ")"がないとエラー
            let then = self.stmt(); //ifの中の式
            if self.consume(TokenKind::ELS) {
                let els = self.stmt();
                return Node::new_node_if(cond, then, els);
            } else {
                return Node::new_node_if(cond, then, Box::new(Node::Nil));
            }
        } else {
            let node = self.expr();
            self.expect(TokenKind::SEMI);
            return node;
        }
    }

    //これの生成規則が何を意味するのかがあんまりわからない
    //生成規則
    //expr = assign
    pub fn expr(&mut self) -> Box<Node> {
        self.assign()
    }

    //assignは代入文、または通常の式を表現する
    //生成規則
    //assign = equality ("=" assign)?
    pub fn assign(&mut self) -> Box<Node> {
        let mut node = self.equality();
    
        loop {
            if self.consume(TokenKind::ASS) { //= 代入文の時
                node = Node::new_node(NodeKind::NDASS, node, self.assign());
            }else{
                return node;
            }
        }
    }

    //==とノットイコールを生成する
    //生成規則
    //equality   = relational ("==" relational | "!=" relational)*
    pub fn equality(&mut self) -> Box<Node> {
        let mut node = self.relational();

        loop {
            if self.consume(TokenKind::EQ) { //==
                node = Node::new_node(NodeKind::NDEQ, node, self.relational());
            }else if self.consume(TokenKind::NEQ) { //ノットイコール
                node = Node::new_node(NodeKind::NDNEQ, node, self.relational());
            }else{
                return node;
            }
        }
    }

    //比較演算子を生成する
    //生成規則
    //relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    pub fn relational(&mut self) -> Box<Node> {
        let mut node = self.add();

        loop {
            if self.consume(TokenKind::LT) {
                node = Node::new_node(NodeKind::NDLT, node, self.add());
            }else if self.consume(TokenKind::LE) {
                node = Node::new_node(NodeKind::NDLE, node, self.add());
            }else if self.consume(TokenKind::GT) {
                node = Node::new_node(NodeKind::NDGT, node, self.add());
            }else if self.consume(TokenKind::GE) {
                node = Node::new_node(NodeKind::NDGE, node, self.add());
            }else{
                return node;
            }
        }
    }

    //足し算、引き算の演算子を生成する
    //生成規則
    //add = mul ("+" mul | "-" mul)*
    pub fn add(&mut self) -> Box<Node> {
        let mut node = self.mul();

        loop {
            if self.consume(TokenKind::ADD) {
                node = Node::new_node(NodeKind::NDADD, node, self.mul());
            }else if self.consume(TokenKind::SUB) {
                node = Node::new_node(NodeKind::NDSUB, node, self.mul());
            }else{
                return node;
            }
        }
    }

    //掛け算、割り算の演算子を生成する
    //生成規則
    //mul = unary ("*" unary | "/" unary)*
    pub fn mul(&mut self) -> Box<Node> {
        let mut node = self.unary();

        loop {
            if self.consume(TokenKind::MUL) {
                node = Node::new_node(NodeKind::NDMUL, node, self.unary());
            }else if self.consume(TokenKind::DIV) {
                node = Node::new_node(NodeKind::NDDIV, node, self.unary());
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
    pub fn unary(&mut self) -> Box<Node> {
        if self.consume(TokenKind::ADD) {
            self.unary()
        }else if self.consume(TokenKind::SUB) {
            Node::new_node(NodeKind::NDSUB, Node::new_node_num(0), self.unary())
        }else{
            self.primary()
        }
    }


    //数字、識別子（変数名）、カッコを生成する
    //生成規則
    //primary = num | ident | "(" expr ")"
    pub fn primary(&mut self) -> Box<Node> {
        if self.consume(TokenKind::LPAR) { //(の時
            let node = self.expr();
            self.expect(TokenKind::RPAR);
            return node;
        }else if self.consume_ident() { //IDの時, ここ汚すぎですね、後で変更するはず
            //新しい変数かどうかをチェック
            let s = self.expect_id(); //トークンの文字列を取得
            let result_find = (&mut self.vec_lvar).iter().find(|v| &v.name == &s); //vec_lvarにあるかどうか
    
            match result_find {
                Some(lvar) => { //変数が存在した場合
                    return Node::new_node_var(lvar.offset)
                },
                None => { //変数が存在しなかった場合
                    let num_lvar = ((self.get_vec_var_len() + 1) * 8) as i32; //現状の変数の数からベースポインタへのオフセットを計算する
    
                    let lvar_to_push = LVar {
                        name: s.clone(),
                        offset: num_lvar,
                    };
                    // vec_lvar.push(lvar_to_push); //変数を追加
                    self.push_vec_var(lvar_to_push);
    
                    return Node::new_node_var(num_lvar);
                }
            }
        }   
        
        else {
            return Node::new_node_num(self.expect_number()); //数字があれば数字を返す、なければエラーを出す
        }
    }
    
}
