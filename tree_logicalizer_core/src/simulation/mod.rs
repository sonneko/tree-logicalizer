// tree_logicalizer_core/src/simulation/mod.rs

use crate::parser::ast::{Ast,ModuleDef};

/// 回路の静的な接続情報（グラフ構造）
#[derive(Debug)]
pub struct SimGraph {
    pub module_defs: std::collections::HashMap<String, ModuleDef>,
}

impl SimGraph {
    // Day 2: ASTを受け取り、モジュール定義をマップに格納する
    pub fn from_ast(ast: Ast) -> Self {
        let mut module_defs = std::collections::HashMap::new();
        for module in ast.modules {
            module_defs.insert(module.name.clone(), module);
        }
        SimGraph { module_defs }
    }
}

/// シミュレーションの動的な状態（ワイヤーの値など）
#[derive(Debug)]
pub struct SimState {
    pub wire_values: std::collections::HashMap<usize, u64>,
}

impl SimState {
    pub fn new() -> Self {
        SimState { wire_values: std::collections::HashMap::new() }
    }
}