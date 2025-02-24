mod token;

use token::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut temp_scope: String = String::new();
    let mut temp_tokens = Vec::new();

    // 一行ずつブロックレベルの解析
    for line in input.lines() {
        let block_tokens = tokenize_block(line, &temp_scope);
        let next_scope = next_block_scope(line, &temp_scope);

        temp_tokens.extend(block_tokens);
        temp_scope = next_scope;

        if !temp_scope.is_empty() {
            continue;
        }

        // 各ブロックトークン内のインライン要素を解析
        for token in temp_tokens.iter() {
            match token {
                Token::UnResolvedText { value } => {
                    let inline_tokens = tokenize_inline(value);
                    tokens.extend(inline_tokens);
                }
                _ => {
                    tokens.push(token.clone());
                }
            }
        }

        if line != input.lines().last().unwrap() {
            tokens.push(Token::Newline);
        }

        temp_tokens.clear();
    }

    // ブロックトークンが残っている場合は閉じる
    if !temp_tokens.is_empty() {
        close_tokens(&mut temp_tokens);
        tokens.extend(temp_tokens);
    }

    tokens
}

fn close_tokens(tokens: &mut Vec<Token>) {
    match tokens.first() {
        Some(Token::CodeBlockOpen { .. }) => {
            tokens.push(Token::CodeBlockClose);
        }
        _ => {}
    }
}

fn next_block_scope(input: &str, scope: &str) -> String {
    if input.starts_with("```") {
        if scope == "code" {
            return "".to_string();
        } else {
            return "code".to_string();
        }
    }
    scope.to_string()
}

fn tokenize_block(input: &str, scope: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let input = input.trim();

    // コードブロックの処理
    if scope == "code" {
        if input.starts_with("```") {
            tokens.push(Token::CodeBlockClose);
        } else {
            tokens.push(Token::CodeBlockText {
                value: input.to_string(),
            });
        }

        return tokens;
    }

    if input.starts_with("```") {
        let sliced = &input[3..];

        if sliced.is_empty() {
            tokens.push(Token::CodeBlockOpen {
                lang: None,
                meta: None,
            });
            return tokens;
        }

        let splitted = sliced.split(' ').collect::<Vec<&str>>();
        let lang = splitted.get(0).map(|s| s.to_string());
        let meta = splitted.get(1).map(|s| s.to_string());

        tokens.push(Token::CodeBlockOpen { lang, meta });
        return tokens;
    }

    // 見出しの処理
    if input.starts_with("#") {
        let level = input.chars().take_while(|c| *c == '#').count();
        let content = input[level..].trim();

        tokens.push(Token::HeadingOpen { level });
        tokens.push(Token::UnResolvedText {
            value: content.to_string(),
        });
        tokens.push(Token::HeadingClose);
        return tokens;
    }

    //　段落の処理
    tokens.push(Token::ParagraphOpen);
    tokens.push(Token::UnResolvedText {
        value: input.to_string(),
    });
    tokens.push(Token::ParagraphClose);

    tokens
}

fn count_asterisks(chars: &[char], start: usize) -> (usize, usize) {
    let mut i = start;
    let len = chars.len();

    // 連続する'*'の数をカウント
    while i < len && chars[i] == '*' {
        i += 1;
    }

    let count = i - start;
    (count, i) // カウント数と新しい位置を返す
}

fn count_backticks(chars: &[char], start: usize) -> (usize, usize) {
    let mut i = start;
    let len = chars.len();

    while i < len && chars[i] == '`' {
        i += 1;
    }

    (i - start, i)
}

fn tokenize_inline(input: &str) -> Vec<Token> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    let len = chars.len();
    let mut acc = String::new();

    let flush_text = |acc: &mut String, tokens: &mut Vec<Token>| {
        if !acc.is_empty() {
            tokens.push(Token::Text { value: acc.clone() });
            acc.clear();
        }
    };

    while i < len {
        match chars.get(i) {
            Some('*') => {
                flush_text(&mut acc, &mut tokens);

                // '*'の数をカウント
                let (count, new_pos) = count_asterisks(&chars, i);
                i = new_pos;

                match count {
                    // Emphasisの処理
                    1 => {
                        let start_index = i;
                        let sliced_str = chars[start_index..].iter().collect::<String>();
                        let found_index = sliced_str.find("*".repeat(count).as_str());

                        if found_index.is_none() {
                            tokens.push(Token::Text {
                                value: "*".repeat(count).to_string(),
                            });
                        }

                        if found_index.is_some() {
                            let end_index = start_index + found_index.unwrap();
                            let inner = chars[start_index..end_index].to_vec();
                            let inner_tokens = tokenize_inline(&inner.iter().collect::<String>());

                            tokens.push(Token::EmphasisOpen);
                            tokens.extend(inner_tokens);
                            tokens.push(Token::EmphasisClose);

                            i = end_index + count;
                        }
                    }
                    // Strongの処理
                    2 => {
                        let start_index = i;
                        let sliced_str = chars[start_index..].iter().collect::<String>();
                        let found_index = sliced_str.find("*".repeat(count).as_str());

                        if found_index.is_none() {
                            tokens.push(Token::Text {
                                value: "*".repeat(count).to_string(),
                            });
                            continue;
                        }

                        if found_index.is_some() {
                            let end_index = start_index + found_index.unwrap();
                            let inner = chars[start_index..end_index].to_vec();
                            let inner_tokens = tokenize_inline(&inner.iter().collect::<String>());

                            tokens.push(Token::StrongOpen);
                            tokens.extend(inner_tokens);
                            tokens.push(Token::StrongClose);

                            i = end_index + count;
                        }
                    }
                    // その他は通常のテキストとして処理
                    _ => {
                        let asterisks: String = "*".repeat(count);
                        tokens.push(Token::Text { value: asterisks });
                    }
                }
            }
            Some('`') => {
                flush_text(&mut acc, &mut tokens);

                let (count, new_pos) = count_backticks(&chars, i);
                i = new_pos;

                match count {
                    // インラインコードの処理
                    1 => {
                        let start_index = i;
                        let sliced_str = chars[start_index..].iter().collect::<String>();
                        let found_index = sliced_str.find("`".repeat(count).as_str());

                        if found_index.is_none() {
                            tokens.push(Token::Text {
                                value: "`".repeat(count).to_string(),
                            });
                        }

                        if found_index.is_some() {
                            let end_index = start_index + found_index.unwrap();
                            let inner = chars[start_index..end_index].to_vec();
                            let inner_tokens = tokenize_inline(&inner.iter().collect::<String>());

                            tokens.push(Token::CodeInlineOpen);
                            tokens.extend(inner_tokens);
                            tokens.push(Token::CodeInlineClose);

                            i = end_index + count;
                        }
                    }
                    // その他は通常のテキストとして処理
                    _ => {
                        let backticks: String = "`".repeat(count);
                        tokens.push(Token::Text { value: backticks });
                    }
                }
            }
            Some(c) => {
                acc.push(*c);
                i += 1;
            }
            None => break,
        }
    }

    flush_text(&mut acc, &mut tokens);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_token() {
        let input = "text";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::ParagraphOpen,
                Token::Text {
                    value: "text".to_string()
                },
                Token::ParagraphClose
            ]
        );
    }

    #[test]
    fn test_text_multiple_lines() {
        let input = "first line\nsecond line";
        let tokens = tokenize(input);

        assert_eq!(
            tokens,
            vec![
                Token::ParagraphOpen,
                Token::Text {
                    value: "first line".to_string()
                },
                Token::ParagraphClose,
                Token::Newline,
                Token::ParagraphOpen,
                Token::Text {
                    value: "second line".to_string()
                },
                Token::ParagraphClose
            ]
        );
    }

    #[test]
    fn test_heading_level_1_token() {
        let input = "# Heading";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::HeadingOpen { level: 1 },
                Token::Text {
                    value: "Heading".to_string()
                },
                Token::HeadingClose
            ]
        );
    }

    #[test]
    fn test_heading_level_2_token() {
        let input = "## Heading";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::HeadingOpen { level: 2 },
                Token::Text {
                    value: "Heading".to_string()
                },
                Token::HeadingClose
            ]
        );
    }

    #[test]
    fn test_emphasis_tokens() {
        let input = "text *emphasis*";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::ParagraphOpen,
                Token::Text {
                    value: "text ".to_string()
                },
                Token::EmphasisOpen,
                Token::Text {
                    value: "emphasis".to_string()
                },
                Token::EmphasisClose,
                Token::ParagraphClose
            ]
        );
    }

    #[test]
    fn test_emphasis_unclosed_tokens() {
        let input = "text *emphasis";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::ParagraphOpen,
                Token::Text {
                    value: "text ".to_string()
                },
                Token::Text {
                    value: "*".to_string()
                },
                Token::Text {
                    value: "emphasis".to_string()
                },
                Token::ParagraphClose
            ]
        );
    }

    #[test]
    fn test_strong_tokens() {
        let input = "text **strong**";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::ParagraphOpen,
                Token::Text {
                    value: "text ".to_string()
                },
                Token::StrongOpen,
                Token::Text {
                    value: "strong".to_string()
                },
                Token::StrongClose,
                Token::ParagraphClose
            ]
        );
    }

    #[test]
    fn test_strong_unclosed_tokens() {
        let input = "text **strong";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::ParagraphOpen,
                Token::Text {
                    value: "text ".to_string()
                },
                Token::Text {
                    value: "**".to_string()
                },
                Token::Text {
                    value: "strong".to_string()
                },
                Token::ParagraphClose
            ]
        );
    }

    #[test]
    fn test_strong_emphasis_nested_tokens() {
        let input = "**outer *inner* outer**";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::ParagraphOpen,
                Token::StrongOpen,
                Token::Text {
                    value: "outer ".to_string()
                },
                Token::EmphasisOpen,
                Token::Text {
                    value: "inner".to_string()
                },
                Token::EmphasisClose,
                Token::Text {
                    value: " outer".to_string()
                },
                Token::StrongClose,
                Token::ParagraphClose
            ]
        );
    }

    // 3つ以上の*が連続している場合はテキストとして処理する
    #[test]
    fn test_three_asterisks() {
        let input = "***asterisks***";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::ParagraphOpen,
                Token::Text {
                    value: "***".to_string()
                },
                Token::Text {
                    value: "asterisks".to_string()
                },
                Token::Text {
                    value: "***".to_string()
                },
                Token::ParagraphClose
            ]
        );
    }

    // インラインコードの処理
    #[test]
    fn test_code_inline_tokens() {
        let input = "`code`";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::ParagraphOpen,
                Token::CodeInlineOpen,
                Token::Text {
                    value: "code".to_string()
                },
                Token::CodeInlineClose,
                Token::ParagraphClose
            ]
        );
    }

    #[test]
    fn test_code_inline_unclosed_tokens() {
        let input = "`inline code";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::ParagraphOpen,
                Token::Text {
                    value: "`".to_string()
                },
                Token::Text {
                    value: "inline code".to_string()
                },
                Token::ParagraphClose
            ]
        );
    }

    #[test]
    fn test_two_backticks_tokens() {
        let input = "``backticks``";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::ParagraphOpen,
                Token::Text {
                    value: "``".to_string()
                },
                Token::Text {
                    value: "backticks".to_string()
                },
                Token::Text {
                    value: "``".to_string()
                },
                Token::ParagraphClose
            ]
        );
    }

    #[test]
    fn test_code_block_tokens() {
        let input = "```\ncode\n```";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::CodeBlockOpen {
                    lang: None,
                    meta: None
                },
                Token::CodeBlockText {
                    value: "code".to_string()
                },
                Token::CodeBlockClose
            ]
        );
    }

    #[test]
    fn test_code_block_with_lang_tokens() {
        let input = "```rust\ncode\n```";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::CodeBlockOpen {
                    lang: Some("rust".to_string()),
                    meta: None
                },
                Token::CodeBlockText {
                    value: "code".to_string()
                },
                Token::CodeBlockClose
            ]
        );
    }

    #[test]
    fn test_code_block_with_meta_tokens() {
        let input = "```rust meta\ncode\n```";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::CodeBlockOpen {
                    lang: Some("rust".to_string()),
                    meta: Some("meta".to_string())
                },
                Token::CodeBlockText {
                    value: "code".to_string()
                },
                Token::CodeBlockClose
            ]
        );
    }

    #[test]
    fn test_code_block_multiline_tokens() {
        let input = "```\ncode1\ncode2\n```";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::CodeBlockOpen {
                    lang: None,
                    meta: None
                },
                Token::CodeBlockText {
                    value: "code1".to_string()
                },
                Token::CodeBlockText {
                    value: "code2".to_string()
                },
                Token::CodeBlockClose
            ]
        );
    }

    #[test]
    fn test_code_block_unclosed_tokens() {
        let input = "```\ncode";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::CodeBlockOpen {
                    lang: None,
                    meta: None
                },
                Token::CodeBlockText {
                    value: "code".to_string()
                },
                Token::CodeBlockClose
            ]
        );
    }
}
