// tree_logicalizer_core/src/parser/ast.rs

#[derive(Debug, PartialEq, Clone)]
pub enum PortType {
    In,
    Out,
    InOut,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Port {
    pub port_type: PortType,
    pub name: String,
    // Noneは単線(wire)、Some(expr)はバス(bus)
    pub width: Option<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(u32),
    Identifier(String),
    BinaryOp { op: String, left: Box<Expr>, right: Box<Expr> },
    // Day 3以降でワイヤーアクセスなどを追加
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    // 例: wire clk; または bus Data[8];
    WireDef { is_bus: bool, name: String, width: Option<Expr> },
    // Day 3以降でインスタンス、接続などを追加
}

#[derive(Debug, PartialEq, Clone)]
pub struct ModuleDef {
    pub name: String,
    // 例: <N, M>
    pub generic_args: Vec<String>,
    // 例: (In A[N], Out S[N])
    pub ports: Vec<Port>,
    // モジュール内部の定義
    pub body: Vec<Statement>, 
}

// DSL全体を表すルート構造体
#[derive(Debug, PartialEq, Clone)]
pub struct Ast {
    pub modules: Vec<ModuleDef>,
}