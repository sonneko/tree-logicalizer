// tree_logicalizer_core/src/lib.rs

use wasm_bindgen::prelude::*;

// 内部モジュールの宣言
pub mod parser;
pub mod simulation; 
// pub mod utils; // Day 3でエラー処理などのために使用

/// Wasmに公開するシミュレータクラス
#[wasm_bindgen]
pub struct CircuitSimulator {
    graph: simulation::SimGraph,
    state: simulation::SimState,
}

/// シミュレーション結果を返すための最小構造体
#[wasm_bindgen]
#[derive(Debug)]
pub struct SimResult {
    #[wasm_bindgen(readonly)]
    pub success: bool,
    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter)]
    log: String,
}

#[wasm_bindgen]
impl CircuitSimulator {
    // Day 2: 拡張されたパースを行い、SimGraphを初期化する
    #[wasm_bindgen(constructor)]
    pub fn new(dsl_code: String) -> Result<CircuitSimulator, JsValue> {
        // 1. パース: DSL -> AST
        let (_, ast) = parser::parse_dsl(&dsl_code)
            .map_err(|e| JsValue::from_str(&format!("Parsing Error: {:?}", e)))?;
        
        // 2. グラフ構築 (Day 3で実装されるロジック)
        // ここではまだresolverがないため、仮のSimGraphとSimStateを生成
        let graph = simulation::SimGraph::from_ast(ast); 
        let state = simulation::SimState::new();

        Ok(CircuitSimulator { graph, state })
    }

    // Day 2: 成功確認のためのテストメソッドを更新
    pub fn get_info(&self) -> String {
        format!("Module definitions parsed: {}", self.graph.module_defs.len())
    }

    // Day 3以降で step_manual, run_cycles などを実装
    pub fn step_manual(&mut self, _inputs_json: &str) -> SimResult {
        SimResult { success: false, log: "Not implemented yet.".to_string() }
    }
}