use crate::block_parser::process_block;
use crate::token::Token;

/// Markdownテキストをトークン化する関数
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut result_tokens = Vec::new();
    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let tokens = process_block(line, &mut lines);
        result_tokens.extend(tokens);

        // 最後の行じゃない場合は改行を追加
        if lines.peek().is_some() {
            result_tokens.push(Token::Newline);
        }
    }

    result_tokens
}
