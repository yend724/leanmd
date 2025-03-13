use crate::token::Token;
use crate::utils::count_target_chars;

/// インラインレベルの要素を処理する関数
pub fn process_inline(block_tokens: Vec<Token>) -> Vec<Token> {
    let mut result_tokens = Vec::new();

    for token in block_tokens {
        match token {
            Token::UnResolvedText { value } => {
                let inline_tokens = process_inline_element(&value);
                result_tokens.extend(inline_tokens);
            }
            _ => {
                result_tokens.push(token);
            }
        }
    }

    result_tokens
}

/// インライン要素を処理する関数
pub fn process_inline_element(input: &str) -> Vec<Token> {
    let chars: Vec<char> = input.chars().collect();
    let mut result_tokens = Vec::new();
    let mut acc_text = String::new();

    let flush_text = |acc_text: &mut String, result_tokens: &mut Vec<Token>| {
        if !acc_text.is_empty() {
            result_tokens.push(Token::Text {
                value: acc_text.to_string(),
            });
            acc_text.clear();
        }
    };

    let mut index = 0;

    while let Some(c) = chars.get(index) {
        let c = *c;
        match c {
            // 強調の処理
            '*' => {
                flush_text(&mut acc_text, &mut result_tokens);
                index = process_emphasis(&chars, index, &mut result_tokens);
                continue;
            }
            // インラインコードの処理
            '`' => {
                flush_text(&mut acc_text, &mut result_tokens);
                index = process_code_inline(&chars, index, &mut result_tokens);
                continue;
            }
            // 画像の処理
            '!' => {
                flush_text(&mut acc_text, &mut result_tokens);
                let (new_index, processed) = process_image(&chars, index, &mut result_tokens);
                index = new_index;
                if processed {
                    continue;
                }
                acc_text.push(c);
            }
            // リンクの処理
            '[' => {
                flush_text(&mut acc_text, &mut result_tokens);
                let (new_index, processed) = process_link(&chars, index, &mut result_tokens);
                index = new_index;
                if processed {
                    continue;
                }
                acc_text.push(c);
            }
            // 通常のテキストの処理
            _ => {
                acc_text.push(c);
            }
        }
        index += 1;
    }

    if !acc_text.is_empty() {
        result_tokens.push(Token::Text { value: acc_text });
    }

    result_tokens
}

/// 強調要素を処理する関数
fn process_emphasis(chars: &[char], start_index: usize, result_tokens: &mut Vec<Token>) -> usize {
    let (count, new_index) = count_target_chars('*', chars, start_index);
    let mut index = new_index;

    match count {
        1 | 2 => {
            let start_index = index;
            let sliced_str = &chars[start_index..].iter().collect::<String>();
            let found_index = sliced_str.find("*".repeat(count).as_str());

            if found_index.is_none() {
                result_tokens.push(Token::Text {
                    value: "*".repeat(count).to_string(),
                });
                return index;
            }

            let end_index = found_index.unwrap();
            let inner = &sliced_str[..end_index];

            let inner_tokens = process_inline_element(inner);

            match count {
                1 => {
                    result_tokens.push(Token::EmphasisOpen);
                    result_tokens.extend(inner_tokens);
                    result_tokens.push(Token::EmphasisClose);
                }
                2 => {
                    result_tokens.push(Token::StrongOpen);
                    result_tokens.extend(inner_tokens);
                    result_tokens.push(Token::StrongClose);
                }
                _ => {
                    // 1 or 2のみなのでここには来ない
                    unreachable!();
                }
            }

            index += inner.chars().count() + count;
        }
        _ => {
            // 3つ以上の*は通常のテキストとして扱う
            result_tokens.push(Token::Text {
                value: "*".repeat(count),
            });
        }
    }

    index
}

/// インラインコードを処理する関数
fn process_code_inline(
    chars: &[char],
    start_index: usize,
    result_tokens: &mut Vec<Token>,
) -> usize {
    let (count, new_index) = count_target_chars('`', chars, start_index);
    let mut index = new_index;

    match count {
        1 => {
            let start_index = index;
            let sliced_str = &chars[start_index..].iter().collect::<String>();
            let found_index = sliced_str.find("`".repeat(count).as_str());

            if found_index.is_none() {
                result_tokens.push(Token::Text {
                    value: "`".repeat(count).to_string(),
                });
                return index;
            }

            let end_index = found_index.unwrap();
            let inner = &sliced_str[..end_index];

            result_tokens.push(Token::CodeInline {
                value: inner.to_string(),
            });

            index += inner.chars().count() + count;
        }
        _ => {
            // 2つ以上の`は通常のテキストとして扱う
            result_tokens.push(Token::Text {
                value: "`".repeat(count),
            });
        }
    }

    index
}

/// 画像を処理する関数
fn process_image(
    chars: &[char],
    start_index: usize,
    result_tokens: &mut Vec<Token>,
) -> (usize, bool) {
    let index = start_index + 1;
    let next_char = chars.get(index);

    // ![の形式でなければ処理しない
    if next_char.is_none() || next_char.unwrap() != &'[' {
        result_tokens.push(Token::Text {
            value: "!".to_string(),
        });
        return (start_index, false);
    }

    // 全体の文字列を取得
    let full_str: String = chars[start_index..].iter().collect();

    // ![alt](url)の形式を正規表現で検出する代わりに、文字列操作で処理
    let alt_start = 2; // ![の後

    // ](を探す
    if let Some(alt_end) = full_str[alt_start..].find("](") {
        let alt_text = &full_str[alt_start..alt_start + alt_end];
        let url_start = alt_start + alt_end + 2; // ](の後

        // 閉じ括弧を探す
        if let Some(url_end) = full_str[url_start..].find(")") {
            let url = &full_str[url_start..url_start + url_end];
            let total_len = url_start + url_end + 1; // 閉じ括弧を含む全体の長さ

            // 画像トークンを追加
            result_tokens.push(Token::Image {
                url: url.to_string(),
                title: None,
                alt: Some(alt_text.to_string()),
            });

            // 残りのテキストがあれば追加
            if total_len < full_str.len() {
                result_tokens.push(Token::Text {
                    value: full_str[total_len..].to_string(),
                });
            }

            return (chars.len(), true);
        }
    }

    // 正しい形式でなければ、元のテキストとして処理
    result_tokens.push(Token::Text { value: full_str });

    (chars.len(), true)
}

/// リンクを処理する関数
fn process_link(
    chars: &[char],
    start_index: usize,
    result_tokens: &mut Vec<Token>,
) -> (usize, bool) {
    // 全体の文字列を取得
    let full_str: String = chars[start_index..].iter().collect();

    // [text](url)の形式を文字列操作で処理
    let text_start = 1; // [の後

    // ](を探す
    if let Some(text_end) = full_str[text_start..].find("](") {
        let link_text = &full_str[text_start..text_start + text_end];
        let url_start = text_start + text_end + 2; // ](の後

        // 閉じ括弧を探す
        if let Some(url_end) = full_str[url_start..].find(")") {
            let url = &full_str[url_start..url_start + url_end];
            let total_len = url_start + url_end + 1; // 閉じ括弧を含む全体の長さ

            // リンクトークンを追加
            let inner_tokens = process_inline_element(link_text);

            result_tokens.push(Token::LinkOpen {
                url: url.to_string(),
                title: None,
            });
            result_tokens.extend(inner_tokens);
            result_tokens.push(Token::LinkClose);

            // 残りのテキストがあれば追加
            if total_len < full_str.len() {
                result_tokens.push(Token::Text {
                    value: full_str[total_len..].to_string(),
                });
            }

            return (chars.len(), true);
        }
    }

    // 正しい形式でなければ、元のテキストとして処理
    result_tokens.push(Token::Text { value: full_str });

    (chars.len(), true)
}
