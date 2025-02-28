use crate::token::Token;

/// インラインレベルの要素を処理するプロセッサー
pub struct InlineProcessor;

impl InlineProcessor {
    /// 新しいインラインプロセッサーを作成
    pub fn new() -> Self {
        Self
    }

    /// 連続するアスタリスクの数をカウント
    fn count_asterisks(&self, chars: &[char], start: usize) -> (usize, usize) {
        let mut i = start;
        let len = chars.len();

        // 連続する'*'の数をカウント
        while i < len && chars[i] == '*' {
            i += 1;
        }

        let count = i - start;
        (count, i) // カウント数と新しい位置を返す
    }

    /// 連続するバッククォートの数をカウント
    fn count_backticks(&self, chars: &[char], start: usize) -> (usize, usize) {
        let mut i = start;
        let len = chars.len();

        while i < len && chars[i] == '`' {
            i += 1;
        }

        (i - start, i)
    }

    /// インラインレベルの要素を処理
    pub fn process_inline(&self, input: &str) -> Vec<Token> {
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
                    let (count, new_pos) = self.count_asterisks(&chars, i);
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
                                let inner_tokens =
                                    self.process_inline(&inner.iter().collect::<String>());

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
                                let inner_tokens =
                                    self.process_inline(&inner.iter().collect::<String>());

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

                    let (count, new_pos) = self.count_backticks(&chars, i);
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
                                let inner_tokens =
                                    self.process_inline(&inner.iter().collect::<String>());

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
}
