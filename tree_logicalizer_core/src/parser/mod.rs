pub mod ast;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, alpha1, digit1},
    combinator::{map, opt, recognize, value},
    multi::{many0, separated_list0},
    sequence::{preceded, delimited, tuple},
};
use crate::parser::ast::{Ast, ModuleDef, Port, PortType, Expr, Statement};

// --- ヘルパー関数 ---

// スペースや改行を無視するためのヘルパー
fn sp<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    preceded(multispace0, inner)
}

// 識別子 (モジュール名、ワイヤー名、ジェネリクス名など)
fn identifier(input: &str) -> IResult<&str, String> {
    map(
        recognize(tuple((alpha1, many0(alt((alpha1, digit1, tag("_"))))))),
        |s: &str| s.to_string(),
    )(input)
}

fn number(input: &str) -> IResult<&str, u32> {
    map(digit1, |s: &str| s.parse::<u32>().unwrap())(input)
}

// --- Expr (式) のパース (Day 2では単なる識別子か数値のみサポート) ---

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    sp(alt((
        map(number, Expr::Number),
        map(identifier, Expr::Identifier),
        // Day 3で二項演算子などを追加
    )))(input)
}

// --- ModuleDef のサブ要素のパース ---

// <N, M> のようなジェネリクス引数をパース
fn parse_generic_args(input: &str) -> IResult<&str, Vec<String>> {
    opt(delimited(
        sp(tag("<")),
        separated_list0(sp(tag(",")), identifier),
        sp(tag(">")),
    ))(input)
    .map(|(input, opt_args)| (input, opt_args.unwrap_or_default()))
}

// In/Out/InOut のポートタイプをパース
fn parse_port_type(input: &str) -> IResult<&str, PortType> {
    sp(alt((
        value(PortType::In, tag("In")),
        value(PortType::Out, tag("Out")),
        value(PortType::InOut, tag("InOut")),
    )))(input)
}

// [N] や [8] のようなバス幅をパース
fn parse_bus_width(input: &str) -> IResult<&str, Option<Expr>> {
    opt(delimited(sp(tag("[")), parse_expr, sp(tag("]"))))(input)
}

// ポート宣言 (例: In A[4]) をパース
fn parse_port(input: &str) -> IResult<&str, Port> {
    map(
        tuple((parse_port_type, identifier, parse_bus_width)),
        |(port_type, name, width)| Port { port_type, name, width },
    )(input)
}

// ポートリスト (例: (In A, Out S)) をパース
fn parse_port_list(input: &str) -> IResult<&str, Vec<Port>> {
    delimited(
        sp(tag("(")),
        separated_list0(sp(tag(",")), parse_port),
        sp(tag(")")),
    )(input)
}

// ワイヤー定義 (例: wire clk; または bus Data[N];) をパース
fn parse_wire_def(input: &str) -> IResult<&str, Statement> {
    map(
        tuple((
            sp(alt((tag("wire"), tag("bus")))), // wire/bus キーワード
            identifier,                        // ワイヤー名
            parse_bus_width,                   // バス幅 (オプション)
            sp(tag(";")),                      // 終端のセミコロン
        )),
        |(type_str, name, width, _)| {
            Statement::WireDef { 
                is_bus: type_str == "bus",
                name,
                width,
            }
        },
    )(input)
}


// --- ModuleDef 全体のパース (Day 2で完成) ---

/// 単一のModuleDefをパース
pub fn parse_module_def(input: &str) -> IResult<&str, ModuleDef> {
    map(
        tuple((
            sp(tag("module")),              // "module" キーワード
            sp(identifier),                     // モジュール名
            parse_generic_args,             // <N> などのジェネリクス
            parse_port_list,                // (In A, Out S) などのポートリスト
            sp(delimited(sp(tag("{")), many0(parse_wire_def), sp(tag("}")))), // ボディ (Day 2ではワイヤー定義のみ)
        )),
        |(_, name, generic_args, ports, body)| {
            ModuleDef { name, generic_args, ports, body }
        },
    )(input)
}

/// DSL全体 (複数のモジュール定義) をパース
pub fn parse_dsl(input: &str) -> IResult<&str, Ast> {
    map(many0(parse_module_def), |modules| Ast { modules })(input)
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{PortType, Expr, Statement};

    // --- ヘルパー関数 ---
    fn test_parse_ok(parser: impl Fn(&str) -> IResult<&str, ModuleDef>, code: &str, expected_name: &str) {
        let result = parser(code);
        if result.is_err() {
            println!("{:?}", result);
        }
        assert!(result.is_ok(), "Parsing failed for code: {}", code);
        let (remaining, module) = result.unwrap();
        assert_eq!(remaining, "", "Did not consume all input.");
        assert_eq!(module.name, expected_name.to_string());
    }

    // --- 基本テスト（Day 1から継続） ---
    #[test]
    fn test_simple_module() {
        test_parse_ok(parse_module_def, "module Empty {}", "Empty");
    }
    
    // --- Day 2: ジェネリクスとポートのテスト ---

    #[test]
    fn test_generic_args() {
        let code = "module Generic<N, M> (In A[N]) {}";
        let (_, module) = parse_module_def(code).unwrap();
        assert_eq!(module.generic_args, vec!["N".to_string(), "M".to_string()]);
    }

    #[test]
    fn test_ports_single_wire() {
        let code = "module SinglePort (In clk, Out data) {}";
        let (_, module) = parse_module_def(code).unwrap();
        
        assert_eq!(module.ports.len(), 2);
        assert_eq!(module.ports[0], Port {
            port_type: PortType::In,
            name: "clk".to_string(),
            width: None,
        });
        assert_eq!(module.ports[1].port_type, PortType::Out);
        assert!(module.ports[1].width.is_none());
    }
    
    #[test]
    fn test_ports_with_bus() {
        let code = "module BusPort (In DataBus[8], Out Result[N]) {}";
        let (_, module) = parse_module_def(code).unwrap();
        
        assert_eq!(module.ports.len(), 2);
        // DataBus[8] のテスト
        assert_eq!(module.ports[0].name, "DataBus");
        assert_eq!(module.ports[0].width, Some(Expr::Number(8)));
        // Result[N] のテスト
        assert_eq!(module.ports[1].name, "Result");
        assert_eq!(module.ports[1].width, Some(Expr::Identifier("N".to_string())));
    }

    // --- Day 2: モジュールボディ（ワイヤー定義）のテスト ---

    #[test]
    fn test_body_wire_defs() {
        let code = "module InternalWires (In A) {
            wire internal_clk;
            bus State[16];
        }";
        let (_, module) = parse_module_def(code).unwrap();
        
        assert_eq!(module.body.len(), 2);
        
        // wire internal_clk;
        assert_eq!(module.body[0], Statement::WireDef {
            is_bus: false,
            name: "internal_clk".to_string(),
            width: None,
        });
        
        // bus State[16];
        assert_eq!(module.body[1], Statement::WireDef {
            is_bus: true,
            name: "State".to_string(),
            width: Some(Expr::Number(16)),
        });
    }

    // --- DSL全体 (Ast) のテスト ---
    #[test]
    fn test_full_dsl_ast() {
        let code = "
            module A {}
            module B<M>(In C[1]) {
                wire X;
            }
        ";
        let (_, ast) = parse_dsl(code).unwrap();
        assert_eq!(ast.modules.len(), 2);
        assert_eq!(ast.modules[0].name, "A");
        assert_eq!(ast.modules[1].name, "B");
        assert_eq!(ast.modules[1].ports.len(), 1);
        assert_eq!(ast.modules[1].body.len(), 1);
    }
}