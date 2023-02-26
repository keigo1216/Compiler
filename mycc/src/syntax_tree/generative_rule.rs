use std::collections::VecDeque;
use crate::token::{Token, TokenKind};
use crate::syntax_tree::{LVar, Node, NodeKind, GenerativeRule};

impl GenerativeRule {
    //非終端記号
    //生成規則を書いてます

    //一つのプログラムの塊は複数のステートメントで成り立っている
    //生成規則
    //program = stmt*
    // *は一つ以上を表す正規表現
    pub fn program(token: &mut VecDeque<Token>) -> VecDeque<Box<Node>> {
        let mut code: VecDeque<Box<Node>> = VecDeque::new(); //構文木を入れるキュー
        let mut vec_lvar: Vec<LVar> = Vec::new(); //ローカル変数の種類とその位置を格納する
        while !GenerativeRule::at_eof(token) {
            code.push_back(GenerativeRule::stmt(token, &mut vec_lvar));
            // GenerativeRule::compound_stmt(token, &mut vec_lvar);x
        }
        code
    }

    pub fn compound_stmt(token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
        let node = *GenerativeRule::stmt(token, vec_lvar);
        if let Node::Elm { kind, lhs, rhs, val, offset, cond, then, els, body , ..} = node {
            if GenerativeRule::at_eof(token) { //eofの時
                return Box::new(Node::Elm { kind, next: Box::new(Node::Nil), lhs, rhs, val, offset, cond, then, els, body });
            } else {
                return Box::new(Node::Elm { kind, next: GenerativeRule::compound_stmt(token, vec_lvar), lhs, rhs, val, offset, cond, then, els, body });
            }
        } else {
            eprintln!("compound_stmt error");
            std::process::exit(1);
        }
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
    pub fn stmt(token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
        if GenerativeRule::consume(token, TokenKind::RETURN) { // "return" expr ";"
            let node = Node::new_node(NodeKind::NDRETURN, GenerativeRule::expr(token, vec_lvar), Box::new(Node::Nil));
            GenerativeRule::expect(token, TokenKind::SEMI);
            return node;
        } else if GenerativeRule::consume(token, TokenKind::IF) { //if文の時
            GenerativeRule::expect(token, TokenKind::LPAR); // "("がないとエラー
            let cond = GenerativeRule::expr(token, vec_lvar); //ifの条件文
            GenerativeRule::expect(token, TokenKind::RPAR); // ")"がないとエラー
            let then = GenerativeRule::stmt(token, vec_lvar); //ifの中の式
            if GenerativeRule::consume(token, TokenKind::ELS) {
                let els = GenerativeRule::stmt(token, vec_lvar);
                return Node::new_node_if(cond, then, els);
            } else {
                return Node::new_node_if(cond, then, Box::new(Node::Nil));
            }
        } else if GenerativeRule::consume(token, TokenKind::LBLOCK) { // "{" stmt* "}"
            let mut code_block: VecDeque<Box<Node>> = VecDeque::new();
            while !GenerativeRule::consume(token, TokenKind::RBLOCK) { // "}"トークンが来るまで
                code_block.push_back(GenerativeRule::stmt(token, vec_lvar));
            }
            return Node::new_node_block(code_block);
        } else {
            let node = GenerativeRule::expr(token, vec_lvar);
            GenerativeRule::expect(token, TokenKind::SEMI);
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
        GenerativeRule::assign(token, vec_lvar)
    }

    //assignは代入文、または通常の式を表現する
    //生成規則
    //assign = equality ("=" assign)?
    pub fn assign(token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
        let mut node = GenerativeRule::equality(token, vec_lvar);

        loop {
            if GenerativeRule::consume(token, TokenKind::ASS) { //= 代入文の時
                node = Node::new_node(NodeKind::NDASS, node, GenerativeRule::assign(token, vec_lvar));
            }else{
                return node;
            }
        }
    }

    //==とノットイコールを生成する
    //生成規則
    //equality   = relational ("==" relational | "!=" relational)*
    pub fn equality(token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
        let mut node = GenerativeRule::relational(token, vec_lvar);

        loop {
            if GenerativeRule::consume(token, TokenKind::EQ) { //==
                node = Node::new_node(NodeKind::NDEQ, node, GenerativeRule::relational(token, vec_lvar));
            }else if GenerativeRule::consume(token, TokenKind::NEQ) { //ノットイコール
                node = Node::new_node(NodeKind::NDNEQ, node, GenerativeRule::relational(token, vec_lvar));
            }else{
                return node;
            }
        }
    }

    //比較演算子を生成する
    //生成規則
    //relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    pub fn relational(token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
        let mut node = GenerativeRule::add(token, vec_lvar);

        loop {
            if GenerativeRule::consume(token, TokenKind::LT) {
                node = Node::new_node(NodeKind::NDLT, node, GenerativeRule::add(token, vec_lvar));
            }else if GenerativeRule::consume(token, TokenKind::LE) {
                node = Node::new_node(NodeKind::NDLE, node, GenerativeRule::add(token, vec_lvar));
            }else if GenerativeRule::consume(token, TokenKind::GT) {
                node = Node::new_node(NodeKind::NDGT, node, GenerativeRule::add(token, vec_lvar));
            }else if GenerativeRule::consume(token, TokenKind::GE) {
                node = Node::new_node(NodeKind::NDGE, node, GenerativeRule::add(token, vec_lvar));
            }else{
                return node;
            }
        }
    }

    //足し算、引き算の演算子を生成する
    //生成規則
    //add = mul ("+" mul | "-" mul)*
    pub fn add (token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
        let mut node = GenerativeRule::mul(token, vec_lvar);

        loop {
            if GenerativeRule::consume(token, TokenKind::ADD) {
                node = Node::new_node(NodeKind::NDADD, node, GenerativeRule::mul(token, vec_lvar));
            }else if GenerativeRule::consume(token, TokenKind::SUB) {
                node = Node::new_node(NodeKind::NDSUB, node, GenerativeRule::mul(token, vec_lvar));
            }else{
                return node;
            }
        }
    }

    //掛け算、割り算の演算子を生成する
    //生成規則
    //mul = unary ("*" unary | "/" unary)*
    pub fn mul(token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
        let mut node = GenerativeRule::unary(token, vec_lvar);

        loop {
            if GenerativeRule::consume(token, TokenKind::MUL) {
                node = Node::new_node(NodeKind::NDMUL, node, GenerativeRule::unary(token, vec_lvar));
            }else if GenerativeRule::consume(token, TokenKind::DIV) {
                node = Node::new_node(NodeKind::NDDIV, node, GenerativeRule::unary(token, vec_lvar));
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
        if GenerativeRule::consume(token, TokenKind::ADD) {
            GenerativeRule::unary(token, vec_lvar)
        }else if GenerativeRule::consume(token, TokenKind::SUB) {
            Node::new_node(NodeKind::NDSUB, Node::new_node_num(0), GenerativeRule::unary(token, vec_lvar))
        }else{
            GenerativeRule::primary(token, vec_lvar)
        }
    }


    //数字、識別子（変数名）、カッコを生成する
    //生成規則
    //primary = num | ident | "(" expr ")"
    pub fn primary(token: &mut VecDeque<Token>, vec_lvar: &mut Vec<LVar>) -> Box<Node> {
        if GenerativeRule::consume(token, TokenKind::LPAR) { //(の時
            let node = GenerativeRule::expr(token, vec_lvar);
            GenerativeRule::expect(token, TokenKind::RPAR);
            return node;
        }else if GenerativeRule::consume_ident(token) { //IDの時, ここ汚すぎですね、後で変更するはず
            //新しい変数かどうかをチェック
            let s = GenerativeRule::expect_id(token); //トークンの文字列を取得
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
            return Node::new_node_num(GenerativeRule::expect_number(token)); //数字があれば数字を返す、なければエラーを出す
        }
    }

    //VecDequeの先頭要素のKindがopと一致したらTrue, それ以外はFalseを返す
    pub fn consume(token: &mut VecDeque<Token>, op: TokenKind) -> bool{
        let front_token = token.pop_front();

        if let Some(t) = front_token { //front_tokenがSome()だったら実行、中身がtに入る
            if t.kind == op {true}
            else {
                token.push_front(t);
                false
            }
        }else{ //front_tokenがNoneだったらエラー
            eprintln!("expect have an element, but it's empty. ");
            std::process::exit(1);
        }
    }

    //先頭のトークンがIDトークンかどうか
    //実際に取り出したりしない
    pub fn consume_ident(token: &mut VecDeque<Token>) -> bool {
        let front_token = token.pop_front(); //先頭のトークンを取得

        match front_token {
            Some(t @ Token { kind:TokenKind::ID, ..}) => {
                token.push_front(t);
                return true;
            }
            Some(t) => {
                token.push_front(t);
                return false;
            }
            None => {
                eprintln!("expect have an element, but it's empty. ");
                std::process::exit(1);
            }
        }
    }

    //primary関数で使う
    //この関数はだいぶ汚いので綺麗に書き直したい
    //トークンがIDならStringを返す
    //それ以外の時はエラーを返す
    //consume_identと一緒に使ってください
    //エラーが出るから変なバグには繋がらないと思います
    //半分ぐらいconsume_identと同じことしているのでまとめたいところ
    pub fn expect_id(token: &mut VecDeque<Token>) -> String {

        let front_token = token.pop_front();

        match front_token {
            Some(Token { kind:TokenKind::ID, str:Some(s), ..}) => { //正常なとき
                return s;
            }
            Some(Token { kind: TokenKind::ID, str:None, ..}) => { //ノードのトークンはIDだが文字列が入っていないとき
                eprintln!("Node token is ID, but don't have an ID String. ");
                std::process::exit(1);
            }
            Some(_) => { //ID以外のノードを持つのが入ってきたとき
                eprintln!("expect have ID token, but don't have. ");
                std::process::exit(1);
            }
            None => { //ノードがNoneのとき
                eprintln!("expect have an element, but it's empty. ");
                std::process::exit(1);
            }
        }
    }

    //VecDequeの先頭要素が数字の時その先頭要素を返し、それ以外の時エラー出力する
    pub fn expect_number(token: &mut VecDeque<Token>) -> i32{
        let front_token = token.pop_front();
        
        match front_token {
            Some(Token { kind: TokenKind::TKNUM, val: Some(_val), ..}) => { //正常なとき
                return _val;
            }
            Some(Token{ kind: TokenKind::TKNUM, val: None, ..}) => { //トークンはNUMだが、数字が空の時
                eprintln!("Node token is TKNUM, but don't have an val. ");
                std::process::exit(1);
            }
            Some(_) => {
                eprintln!("expect have TKNUM, but don't have. ");
                std::process::exit(1);
            }
            None => {
                eprintln!("expect have an element, but it's empty. ");
                std::process::exit(1);
            }
        }
    }

    //VecDequeの先頭要素のKindがopと一致していなかったらエラー出力する
    //全てのパターンに一致しなかったトークン用
    pub fn expect(token: &mut VecDeque<Token>, op: TokenKind) {
        let front_token = token.pop_front();

        if let Some(t) = front_token {
            if t.kind != op {
                eprintln!("expect have {:?}, but you have {:?}. ", op, t.kind); 
                std::process::exit(1); 
            }
        }else {
            eprintln!("expect have an element, but it's empty. ");
            std::process::exit(1);
        }

    }

    pub fn at_eof(token: &mut VecDeque<Token>) -> bool {
        let front_token = token.pop_front();

        match front_token {
            Some(Token{ kind:TokenKind::TKEOF, ..}) => { //コードの終端のとき
                return true;
            }
            Some(t) => { //終端ではなかったとき
                token.push_front(t);
                return false;
            }
            None => { //トークンが存在しなかったとき
                eprintln!("expect have an element, but it's empty. ");
                std::process::exit(1);
            }
        }
    }
}