use parser::Root;
use wasm_bindgen::prelude::*;

/// Markdownテキストを解析してASTに変換する関数
fn parse(input: &str) -> Root {
    parser::parse_to_ast(input)
}

/// Markdownテキストを解析してJSON文字列に変換する関数
fn parse_to_ast_json(input: &str) -> String {
    let ast = parse(input);
    serde_json::to_string(&ast).unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn parse_markdown(input: &str) -> String {
    parse_to_ast_json(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_json() {
        let json = parse_to_ast_json("# Hello World");
        assert_eq!(
            json,
            "{\"children\":[{\"Heading\":{\"depth\":1,\"children\":[{\"Text\":{\"value\":\"Hello World\"}}]}}]}"
        );
    }
}
