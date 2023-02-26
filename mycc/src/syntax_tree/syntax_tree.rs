

// impl SyntaxTree {
//     //コンストラクタ
//     pub fn new(token: VecDeque<Token>) -> SyntaxTree{
//         SyntaxTree{
//             vec_lvar: Vec::new(),
//             token,
//             code: VecDeque::new(),
//         }
//     }

//     //vec_varの長さを取得するラッパー
//     pub fn get_vec_var_len(&self) -> usize {
//         return self.vec_lvar.len();
//     }

//     //vec_varに要素をpushするラッパー
//     pub fn push_vec_var(&mut self, lvar: LVar) {
//         self.vec_lvar.push(lvar);
//     }
// }