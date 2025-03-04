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
            // 順序なしリストの処理
            input if input.starts_with("- ") => {
                let content = input.strip_prefix("- ").unwrap_or("");

                // リストの開始トークンを追加
                result_tokens.push(Token::UnorderedListOpen);

                // 最初のリストアイテムを処理
                result_tokens.push(Token::ListItemOpen);
                let inline_tokens = self.process_inline(vec![Token::UnResolvedText {
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
                            let item_tokens = self.process_inline(vec![Token::UnResolvedText {
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
            // 順序ありリストの処理
            input if self.is_ordered_list_item(input) => {
                // 開始番号を取得
                let (start, content) = self.extract_ordered_list_item(input);

                // リストの開始トークンを追加
                result_tokens.push(Token::OrderedListOpen { start });

                // 最初のリストアイテムを処理
                result_tokens.push(Token::ListItemOpen);
                let inline_tokens = self.process_inline(vec![Token::UnResolvedText {
                    value: content.to_string(),
                }]);
                result_tokens.extend(inline_tokens);
                result_tokens.push(Token::ListItemClose);

                // 連続するリストアイテムを処理
                let peek_lines = lines.clone();
                let mut consecutive_items = 0;

                for next_line in peek_lines {
                    if self.is_ordered_list_item(next_line) {
                        consecutive_items += 1;
                    } else {
                        break;
                    }
                }

                // 連続するリストアイテムを消費
                for _ in 0..consecutive_items {
                    if let Some(line) = lines.next() {
                        if self.is_ordered_list_item(line) {
                            let (_, item_content) = self.extract_ordered_list_item(line);
                            result_tokens.push(Token::Newline);
                            result_tokens.push(Token::ListItemOpen);
                            let item_tokens = self.process_inline(vec![Token::UnResolvedText {
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

                                match count {
                                    1 => {
                                        result_tokens.push(Token::CodeInline {
                                            value: String::from_iter(inner.iter()),
                                        });
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
                // 画像の処理
                '!' => {
                    index += 1;
                    let next_char = chars.get(index);

                    if next_char.is_none() || *next_char.unwrap() != '[' {
                        result_tokens.push(Token::Text {
                            value: "!".to_string(),
                        });
                        continue;
                    }

                    let alt_start_index = index + 1;
                    let alt_end_index = &input.find("](");

                    if alt_end_index.is_none() {
                        result_tokens.push(Token::Text {
                            value: "!".to_string(),
                        });
                        continue;
                    }

                    let alt = &input[alt_start_index..alt_end_index.unwrap()];

                    let url_start_index = alt_end_index.unwrap() + 2;
                    let url_end_index = &input.find(")");

                    if url_end_index.is_none() {
                        result_tokens.push(Token::Text {
                            value: "!".to_string(),
                        });
                        continue;
                    }

                    let url = &input[url_start_index..url_end_index.unwrap()];

                    // 画像として処理する前にテキストをフラッシュ
                    flush_text(&mut acc_text, &mut result_tokens);

                    result_tokens.push(Token::Image {
                        url: url.to_string(),
                        title: None,
                        alt: Some(alt.to_string()),
                    });

                    index = url_end_index.unwrap() + 1;

                    continue;
                }
                // リンクの処理
                '[' => {
                    index += 1;

                    let text_start_index = index;
                    let text_end_index = &input.find("](");

                    if text_end_index.is_none() {
                        acc_text.push(c);
                        continue;
                    }

                    let text = &input[text_start_index..text_end_index.unwrap()];

                    let url_start_index = text_end_index.unwrap() + 2;
                    let url_end_index = &input.find(")");

                    if url_end_index.is_none() {
                        acc_text.push(c);
                        continue;
                    }

                    // リンクとして処理する前にテキストをフラッシュ
                    flush_text(&mut acc_text, &mut result_tokens);

                    let inner_tokens = self.process_inline_element(text);
                    let url = &input[url_start_index..url_end_index.unwrap()];

                    result_tokens.push(Token::LinkOpen {
                        url: url.to_string(),
                        title: None,
                    });
                    result_tokens.extend(inner_tokens);
                    result_tokens.push(Token::LinkClose);

                    index = url_end_index.unwrap() + 1;

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

    // 順序ありリストアイテムかどうかを判定
    fn is_ordered_list_item(&self, input: &str) -> bool {
        // 数字で始まり、その後に「. 」が続く場合に順序ありリストと判定
        let mut chars = input.chars();
        let mut has_digit = false;

        // 先頭の数字を確認
        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                has_digit = true;
            } else if c == '.' {
                // 数字の後にピリオドがある
                if has_digit && chars.next().map_or(false, |next| next.is_whitespace()) {
                    return true;
                }
                return false;
            } else {
                return false;
            }
        }

        false
    }

    // 順序ありリストアイテムから開始番号とコンテンツを抽出
    fn extract_ordered_list_item<'a>(&self, input: &'a str) -> (u32, &'a str) {
        let mut number_end = 0;
        let mut start = 1;

        // 数字部分を抽出
        for (i, c) in input.char_indices() {
            if c.is_ascii_digit() {
                continue;
            } else if c == '.' {
                number_end = i;
                if let Ok(num) = input[..number_end].parse::<u32>() {
                    start = num;
                }
                break;
            } else {
                break;
            }
        }

        // コンテンツ部分を抽出（「. 」の後）
        if number_end > 0 && number_end + 2 <= input.len() {
            let content = input[number_end + 2..].trim_start();
            (start, content)
        } else {
            (start, "")
        }
    }
}
