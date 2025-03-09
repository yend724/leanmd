use parser::Root;
use wasm_bindgen::prelude::*;

/// Markdownテキストを解析してASTに変換する関数
fn parse(input: &str) -> Root {
    parser::parse(input)
}

/// Markdownテキストを解析してJSON文字列に変換する関数
fn parse_to_json(input: &str) -> String {
    let ast = parse(input);
    serde_json::to_string(&ast).unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn parse_markdown(input: &str) -> String {
    parse_to_json(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_json() {
        let json = parse_to_json("# Hello World");
        assert!(json.contains("Heading"));
        assert!(json.contains("depth"));
        assert!(json.contains("1"));
    }
}
