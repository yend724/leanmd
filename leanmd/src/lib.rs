use wasm_bindgen::prelude::*;

/// Markdownテキストを解析してJSON文字列に変換する関数
#[wasm_bindgen(js_name = markdownToJSON)]
pub fn markdown_to_json(input: &str) -> String {
    let ast = parser::markdown_to_ast(input);
    serde_json::to_string(&ast).unwrap_or_else(|_| "{}".to_string())
}

/// Markdownテキストを解析して整形されたJSON文字列に変換する関数
#[wasm_bindgen(js_name = markdownToJSONPretty)]
pub fn markdown_to_json_pretty(input: &str) -> String {
    let ast = parser::markdown_to_ast(input);
    serde_json::to_string_pretty(&ast).unwrap_or_else(|_| "{}".to_string())
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

    #[test]
    fn test_parse_to_json_pretty() {
        let json = markdown_to_json_pretty("# Hello World");
        assert_eq!(json, "{\n  \"children\": [\n    {\n      \"Heading\": {\n        \"depth\": 1,\n        \"children\": [\n          {\n            \"Text\": {\n              \"value\": \"Hello World\"\n            }\n          }\n        ]\n      }\n    }\n  ]\n}");
    }
}
