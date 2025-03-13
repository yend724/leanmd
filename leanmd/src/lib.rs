use wasm_bindgen::prelude::*;

/// Markdownテキストを解析してJSON文字列に変換する関数
#[wasm_bindgen]
pub fn markdown_to_json(input: &str) -> String {
    let ast = parser::markdown_to_ast(input);
    serde_json::to_string(&ast).unwrap_or_else(|_| "{}".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_json() {
        let json = markdown_to_json("# Hello World");
        assert_eq!(
            json,
            "{\"children\":[{\"Heading\":{\"depth\":1,\"children\":[{\"Text\":{\"value\":\"Hello World\"}}]}}]}"
        );
    }
}
