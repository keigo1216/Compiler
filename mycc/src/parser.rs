pub mod stack_assembly;
pub mod codegen;

// 構文木からプログラムを生成する構造体
pub struct CodeGen {
    label_count: i32 // ラベルにつける番号
}