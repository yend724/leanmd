use crate::inline_parser::process_inline;
use crate::token::Token;
use crate::utils::{extract_ordered_list_item, is_ordered_list_item};
use std::iter::Peekable;
use std::str::Lines;

/// ブロックレベルの要素を処理する関数
pub fn process_block(input: &str, lines: &mut Peekable<Lines<'_>>) -> Vec<Token> {
    let mut result_tokens = Vec::new();

    match input {
        // 順序なしリストの処理
        input if input.starts_with("- ") => {
            process_unordered_list(input, lines, &mut result_tokens)
        }
        // 順序ありリストの処理
        input if is_ordered_list_item(input) => {
            process_ordered_list(input, lines, &mut result_tokens)
        }
        // コードブロックの処理
        input if input.starts_with("```") => process_code_block(input, lines, &mut result_tokens),
        // 引用の処理
        input if input.starts_with('>') => process_blockquote(input, lines, &mut result_tokens),
        // 見出しの処理
        input if input.starts_with('#') => process_heading(input, &mut result_tokens),
        // 水平線の処理
        input if input.starts_with("---") => {
            result_tokens.push(Token::ThematicBreak);
        }
        // 段落の処理
        _ => process_paragraph(input, &mut result_tokens),
    }

    result_tokens
}

/// 順序なしリストを処理する関数
fn process_unordered_list(
    input: &str,
    lines: &mut Peekable<Lines<'_>>,
    result_tokens: &mut Vec<Token>,
) {
    let content = input.strip_prefix("- ").unwrap_or("");

    // リストの開始トークンを追加
    result_tokens.push(Token::UnorderedListOpen);

    // 最初のリストアイテムを処理
    result_tokens.push(Token::ListItemOpen);
    let inline_tokens = process_inline(vec![Token::UnResolvedText {
        value: content.to_string(),
    }]);
    result_tokens.extend(inline_tokens);
    result_tokens.push(Token::ListItemClose);

    // 連続するリストアイテムを処理
    let peek_lines = lines.clone();
    let mut consecutive_items = 0;

    for next_line in peek_lines {
        if next_line.starts_with("- ") {
            consecutive_items += 1;
        } else {
            break;
        }
    }

    // 連続するリストアイテムを消費
    for _ in 0..consecutive_items {
        if let Some(line) = lines.next() {
            if line.starts_with("- ") {
                let item_content = line.strip_prefix("- ").unwrap_or("");
                result_tokens.push(Token::Newline);
                result_tokens.push(Token::ListItemOpen);
                let item_tokens = process_inline(vec![Token::UnResolvedText {
                    value: item_content.to_string(),
                }]);
                result_tokens.extend(item_tokens);
                result_tokens.push(Token::ListItemClose);
            }
        }
    }

    // リストの終了トークンを追加
    result_tokens.push(Token::UnorderedListClose);
}

/// 順序ありリストを処理する関数
fn process_ordered_list(
    input: &str,
    lines: &mut Peekable<Lines<'_>>,
    result_tokens: &mut Vec<Token>,
) {
    // 開始番号を取得
    let (start, content) = extract_ordered_list_item(input);

    // リストの開始トークンを追加
    result_tokens.push(Token::OrderedListOpen { start });

    // 最初のリストアイテムを処理
    result_tokens.push(Token::ListItemOpen);
    let inline_tokens = process_inline(vec![Token::UnResolvedText {
        value: content.to_string(),
    }]);
    result_tokens.extend(inline_tokens);
    result_tokens.push(Token::ListItemClose);

    // 連続するリストアイテムを処理
    let peek_lines = lines.clone();
    let mut consecutive_items = 0;

    for next_line in peek_lines {
        if is_ordered_list_item(next_line) {
            consecutive_items += 1;
        } else {
            break;
        }
    }

    // 連続するリストアイテムを消費
    for _ in 0..consecutive_items {
        if let Some(line) = lines.next() {
            if is_ordered_list_item(line) {
                let (_, item_content) = extract_ordered_list_item(line);
                result_tokens.push(Token::Newline);
                result_tokens.push(Token::ListItemOpen);
                let item_tokens = process_inline(vec![Token::UnResolvedText {
                    value: item_content.to_string(),
                }]);
                result_tokens.extend(item_tokens);
                result_tokens.push(Token::ListItemClose);
            }
        }
    }

    // リストの終了トークンを追加
    result_tokens.push(Token::OrderedListClose);
}

/// コードブロックを処理する関数
fn process_code_block(
    input: &str,
    lines: &mut Peekable<Lines<'_>>,
    result_tokens: &mut Vec<Token>,
) {
    let mut code_lines = Vec::new();

    // ```プレフィックスを削除して言語とメタ情報を抽出
    let sliced = input.strip_prefix("```").unwrap_or("");

    // 言語とメタ情報を取得
    let (lang, meta) = if sliced.is_empty() {
        (None, None)
    } else {
        let parts: Vec<&str> = sliced.splitn(2, ' ').collect();
        (
            parts.first().map(|s| s.to_string()),
            parts.get(1).map(|s| s.to_string()),
        )
    };

    // コードブロックの終わりまで行を消費
    for line in lines.take_while(|line| !line.trim().starts_with("```")) {
        code_lines.push(line);
    }

    result_tokens.push(Token::CodeBlockOpen { lang, meta });
    result_tokens.push(Token::CodeBlockText {
        value: code_lines.join("\n"),
    });
    result_tokens.push(Token::CodeBlockClose);
}

/// ブロック引用を処理する関数
fn process_blockquote(
    input: &str,
    lines: &mut Peekable<Lines<'_>>,
    result_tokens: &mut Vec<Token>,
) {
    let remaining = input[1..].trim();
    let inner_tokens = process_block(remaining, lines);

    result_tokens.push(Token::BlockquoteOpen);
    result_tokens.extend(inner_tokens);
    result_tokens.push(Token::BlockquoteClose);
}

/// 見出しを処理する関数
fn process_heading(input: &str, result_tokens: &mut Vec<Token>) {
    let level = input.chars().take_while(|c| *c == '#').count();
    let remaining = input[level..].trim();

    let inline_tokens = process_inline(vec![Token::UnResolvedText {
        value: remaining.to_string(),
    }]);

    result_tokens.push(Token::HeadingOpen { level });
    result_tokens.extend(inline_tokens);
    result_tokens.push(Token::HeadingClose);
}

/// 段落を処理する関数
fn process_paragraph(input: &str, result_tokens: &mut Vec<Token>) {
    let inline_tokens = process_inline(vec![Token::UnResolvedText {
        value: input.to_string(),
    }]);

    result_tokens.push(Token::ParagraphOpen);
    result_tokens.extend(inline_tokens);
    result_tokens.push(Token::ParagraphClose);
}
