use crate::token::Token;
use std::iter::Peekable;
use std::str::Lines;

/// Markdownテキストをトークン化するためのトークナイザー
pub struct Tokenizer {}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer {
    /// 新しいトークナイザーを作成
    pub fn new() -> Self {
        Self {}
    }

    /// Markdownテキストをトークン化
    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut result_tokens = Vec::new();
        let mut lines = input.lines().peekable();

        while let Some(line) = lines.next() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let tokens = self.process_block(line, &mut lines);
            result_tokens.extend(tokens);

            // 最後の行じゃない場合は改行を追加
            if lines.peek().is_some() {
                result_tokens.push(Token::Newline);
            }
        }

        result_tokens
    }

    fn process_block(&self, input: &str, lines: &mut Peekable<Lines<'_>>) -> Vec<Token> {
        let mut result_tokens = Vec::new();

        match input {
            // コードブロックの処理
            input if input.starts_with("```") => {
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
            // 引用の処理
            input if input.starts_with('>') => {
                let remaining = input[1..].trim();
                let inner_tokens = self.process_block(remaining, lines);

                result_tokens.push(Token::BlockquoteOpen);
                result_tokens.extend(inner_tokens);
                result_tokens.push(Token::BlockquoteClose);
            }
            // 見出しの処理
            input if input.starts_with('#') => {
                let level = input.chars().take_while(|c| *c == '#').count();
                let remaining = input[level..].trim();

                let inline_tokens = self.process_inline(vec![Token::UnResolvedText {
                    value: remaining.to_string(),
                }]);

                result_tokens.push(Token::HeadingOpen { level });
                result_tokens.extend(inline_tokens);
                result_tokens.push(Token::HeadingClose);
            }
            // 水平線の処理
            input if input.starts_with("---") => {
                result_tokens.push(Token::ThematicBreak);
            }
            // 段落の処理
            _ => {
                let inline_tokens = self.process_inline(vec![Token::UnResolvedText {
                    value: input.to_string(),
                }]);

                result_tokens.push(Token::ParagraphOpen);
                result_tokens.extend(inline_tokens);
                result_tokens.push(Token::ParagraphClose);
            }
        }

        result_tokens
    }

    fn process_inline(&self, block_tokens: Vec<Token>) -> Vec<Token> {
        let mut result_tokens = Vec::new();

        for token in block_tokens {
            match token {
                Token::UnResolvedText { value } => {
                    let inline_tokens = self.process_inline_element(&value);
                    result_tokens.extend(inline_tokens);
                }
                _ => {
                    result_tokens.push(token.clone());
                }
            }
        }

        result_tokens
    }

    fn process_inline_element(&self, input: &str) -> Vec<Token> {
        let chars: Vec<char> = input.chars().collect();
        // let mut chars_iter = chars.peekable();
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

                    let (count, new_index) = self.count_target_chars('*', &chars, index);
                    index = new_index;

                    match count {
                        1 | 2 => {
                            let start_index = index;
                            let sliced_str = chars[start_index..].iter().collect::<String>();
                            let found_index = sliced_str.find("*".repeat(count).as_str());

                            if found_index.is_none() {
                                result_tokens.push(Token::Text {
                                    value: "*".repeat(count).to_string(),
                                });
                            }

                            if found_index.is_some() {
                                let end_index = start_index + found_index.unwrap();
                                let inner = &chars[start_index..end_index];
                                let inner_tokens =
                                    self.process_inline_element(&String::from_iter(inner.iter()));

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

                                index = end_index + count;
                            }
                        }
                        _ => {
                            // 3つ以上の*は通常のテキストとして扱う
                            result_tokens.push(Token::Text {
                                value: "*".repeat(count),
                            });
                        }
                    }
                    continue;
                }
                // インラインコードの処理
                '`' => {
                    flush_text(&mut acc_text, &mut result_tokens);

                    let (count, new_index) = self.count_target_chars('`', &chars, index);
                    index = new_index;

                    match count {
                        1 => {
                            let start_index = index;
                            let sliced_str = chars[start_index..].iter().collect::<String>();
                            let found_index = sliced_str.find("`".repeat(count).as_str());

                            if found_index.is_none() {
                                result_tokens.push(Token::Text {
                                    value: "`".repeat(count).to_string(),
                                });
                            }

                            if found_index.is_some() {
                                let end_index = start_index + found_index.unwrap();
                                let inner = &chars[start_index..end_index];
                                let inner_tokens =
                                    self.process_inline_element(&String::from_iter(inner.iter()));

                                match count {
                                    1 => {
                                        result_tokens.push(Token::CodeInlineOpen);
                                        result_tokens.extend(inner_tokens);
                                        result_tokens.push(Token::CodeInlineClose);
                                    }
                                    _ => {
                                        // 1 or 2のみなのでここには来ない
                                        unreachable!();
                                    }
                                }

                                index = end_index + count;
                            }
                        }
                        _ => {
                            // 2つ以上の`は通常のテキストとして扱う
                            result_tokens.push(Token::Text {
                                value: "`".repeat(count),
                            });
                        }
                    }
                    continue;
                }
                // 通常のテキストの処理
                _ => {
                    acc_text.push(c);
                }
            }
            index += 1;
        }

        if !acc_text.is_empty() {
            result_tokens.push(Token::Text {
                value: acc_text.to_string(),
            });
        }

        result_tokens
    }

    fn count_target_chars(&self, target: char, chars: &[char], start: usize) -> (usize, usize) {
        let mut i = start;
        let len = chars.len();

        // 連続する'*'の数をカウント
        while i < len && chars[i] == target {
            i += 1;
        }

        let count = i - start;
        (count, i) // カウント数と新しい位置を返す
    }
}
