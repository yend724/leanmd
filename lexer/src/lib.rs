mod token;

use token::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    // 一行ずつブロックレベルの解析
    for line in input.lines() {
        let mut block_tokens = tokenize_block(line);

        // 各ブロックトークン内のインライン要素を解析
        for token in block_tokens.iter_mut() {
            match token {
                Token::Text { value } => {
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
    }

    tokens
}

fn tokenize_block(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let input = input.trim();

    match input.chars().next() {
        Some('#') => {
            let level = input.chars().take_while(|c| *c == '#').count();
            let content = input[level..].trim();

            tokens.push(Token::HeadingOpen { level });
            tokens.push(Token::Text {
                value: content.to_string(),
            });
            tokens.push(Token::HeadingClose);
        }
        _ => {
            tokens.push(Token::Text {
                value: input.to_string(),
            });
        }
    }

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
            vec![Token::Text {
                value: "text".to_string()
            }]
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
                Token::Text {
                    value: "text ".to_string()
                },
                Token::EmphasisOpen,
                Token::Text {
                    value: "emphasis".to_string()
                },
                Token::EmphasisClose
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
                Token::Text {
                    value: "text ".to_string()
                },
                Token::Text {
                    value: "*".to_string()
                },
                Token::Text {
                    value: "emphasis".to_string()
                },
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
                Token::Text {
                    value: "text ".to_string()
                },
                Token::StrongOpen,
                Token::Text {
                    value: "strong".to_string()
                },
                Token::StrongClose
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
                Token::Text {
                    value: "text ".to_string()
                },
                Token::Text {
                    value: "**".to_string()
                },
                Token::Text {
                    value: "strong".to_string()
                }
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
                Token::StrongClose
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
                Token::Text {
                    value: "***".to_string()
                },
                Token::Text {
                    value: "asterisks".to_string()
                },
                Token::Text {
                    value: "***".to_string()
                }
            ]
        );
    }

    #[test]
    fn test_newline() {
        let input = "first line\nsecond line";
        let tokens = tokenize(input);

        assert_eq!(
            tokens,
            vec![
                Token::Text {
                    value: "first line".to_string()
                },
                Token::Newline,
                Token::Text {
                    value: "second line".to_string()
                }
            ]
        );
    }
}
