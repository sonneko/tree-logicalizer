use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, multispace0},
    sequence::{preceded, delimited},
    combinator::map,
};

// 最小AST (抽象構文木)
#[derive(Debug, PartialEq)]
pub struct ModuleDef {
    pub name: String,
    // Day 2以降でポート、ボディなどを追加
}

// スペースや改行を無視するためのヘルパー
fn sp<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    preceded(multispace0, inner)
}

// 識別子 (モジュール名など)
fn identifier(input: &str) -> IResult<&str, String> {
    map(
        take_while1(|c: char| c.is_alphanumeric() || c == '_'),
        |s: &str| s.to_string(),
    )(input)
}

/// 最小モジュール定義のパース: module MyMod {}
pub fn parse_module_def(input: &str) -> IResult<&str, ModuleDef> {
    // "module" キーワードをパース
    let (input, _) = sp(tag("module"))(input)?;

    // モジュール名をパース
    let (input, name) = sp(identifier)(input)?;

    // {} の中身を空としてパース
    let (input, _) = sp(tag("{"))(input)?;
    let (input, _) = sp(tag("}"))(input)?;

    // パース結果をModuleDef構造体にマップ
    Ok((input, ModuleDef { name }))
}

// 最小限のテスト (動作確認用)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_module_parse() {
        let code = "module MyCircuit {}";
        let result = parse_module_def(code);
        assert_eq!(result, Ok(("", ModuleDef { name: "MyCircuit".to_string() })));
    }

    #[test]
    fn test_module_with_spacing() {
        let code = "  module  Another_One\n { \t} ";
        let result = parse_module_def(code);
        assert_eq!(result, Ok(("", ModuleDef { name: "Another_One".to_string() })));
    }

    #[test]
    fn test_invalid_module() {
        let code = "module 123Invalid {}";
        let result = parse_module_def(code);
        assert!(result.is_err());
    }
}