use std::collections::VecDeque;
use crate::syntax_tree::{Obj, Node, Function};

impl Function {
    //コンストラクタ
    pub fn new(body: VecDeque<Box<Node>>, locals: Vec<Obj>) -> Function {
        Function { 
            body, 
            locals,
            stack_size: 0 
        }
    }
}