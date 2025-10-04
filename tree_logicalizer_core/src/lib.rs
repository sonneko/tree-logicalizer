use wasm_bindgen::prelude::*;

// 内部モジュールの宣言
pub mod parser;
// pub mod resolver; // Day 2以降
// pub mod simulation; // Day 2以降

/// Wasmに公開するシミュレータクラス
#[wasm_bindgen]
pub struct CircuitSimulator {
    // ... 内部状態はDay 2以降で定義
    dsl_code: String,
}

#[wasm_bindgen]
impl CircuitSimulator {
    // Day 1: とりあえずコードを受け取るだけの最小実装
    #[wasm_bindgen(constructor)]
    pub fn new(dsl_code: String) -> Result<CircuitSimulator, JsValue> {
        // Day 1の目標: ここで最小限のパースを行い、成功確認をする
        match parser::parse_module_def(&dsl_code) {
            Ok(_) => {
                // パース成功
                Ok(CircuitSimulator { dsl_code })
            }
            Err(e) => {
                // パースエラーをJsValueとして返す
                let err_msg = format!("Parsing Error: {:?}", e);
                Err(JsValue::from_str(&err_msg))
            }
        }
    }

    // Day 1: テスト用のHello World関数
    pub fn greet(&self) -> String {
        format!("Hello from Rust! DSL code length: {}", self.dsl_code.len())
    }

    // ... 他のメソッド (step_manual, get_viewer_dataなど) はDay 2以降に実装
}