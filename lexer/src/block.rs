use crate::token::Token;

/// ブロックレベルの要素を処理するプロセッサー
pub struct BlockProcessor;

impl BlockProcessor {
    /// 新しいブロックプロセッサーを作成
    pub fn new() -> Self {
        Self
    }

    /// ブロックトークンを閉じる
    pub fn close_tokens(&self, tokens: &mut Vec<Token>) {
        if let Some(Token::CodeBlockOpen { .. }) = tokens.first() {
            tokens.push(Token::CodeBlockClose);
        }
    }

    /// 次のブロックスコープを決定
    pub fn next_block_scope(&self, input: &str, scope: &str) -> String {
        if input.starts_with("```") {
            if scope == "code" {
                return "".to_string();
            } else {
                return "code".to_string();
            }
        }
        scope.to_string()
    }

    /// ブロックレベルの要素を処理
    pub fn process_block(&self, input: &str, scope: &str) -> Vec<Token> {
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
            let sliced = input.strip_prefix("```").unwrap();

            if sliced.is_empty() {
                tokens.push(Token::CodeBlockOpen {
                    lang: None,
                    meta: None,
                });
                return tokens;
            }

            let splitted = sliced.split(' ').collect::<Vec<&str>>();
            let lang = splitted.first().map(|s| s.to_string());
            let meta = splitted.get(1).map(|s| s.to_string());

            tokens.push(Token::CodeBlockOpen { lang, meta });
            return tokens;
        }

        // 引用の処理
        if input.starts_with(">") {
            let content = input.strip_prefix(">").unwrap().trim();

            tokens.push(Token::BlockquoteOpen);

            // 引用内のコンテンツを再帰的に処理
            if !content.is_empty() {
                // 引用内のコンテンツに対して再帰的にprocess_blockを呼び出す
                let inner_tokens = self.process_inner_block(content);

                // 内部トークンが単純なテキストのみの場合は段落でラップする
                let is_simple_text = inner_tokens.iter().all(|token| {
                    matches!(
                        token,
                        Token::ParagraphOpen | Token::ParagraphClose | Token::UnResolvedText { .. }
                    )
                });

                if is_simple_text {
                    // 段落タグを保持し、テキストを段落でラップする
                    tokens.push(Token::ParagraphOpen);

                    // BlockquoteOpen/Closeを除外して内部トークンのみを追加
                    for token in inner_tokens.iter() {
                        match token {
                            Token::ParagraphOpen | Token::ParagraphClose => {
                                // 段落タグは省略
                            }
                            Token::UnResolvedText { value: _ } => {
                                // テキストはそのまま追加
                                tokens.push(token.clone());
                            }
                            _ => {
                                // その他のトークンはそのまま追加
                                tokens.push(token.clone());
                            }
                        }
                    }

                    tokens.push(Token::ParagraphClose);
                } else {
                    // 複雑な内容の場合は以前の処理を維持
                    // BlockquoteOpen/Closeを除外して内部トークンのみを追加
                    for token in inner_tokens.iter() {
                        match token {
                            Token::ParagraphOpen | Token::ParagraphClose => {
                                // 段落タグは省略（引用内の段落は特別扱いしない）
                            }
                            Token::UnResolvedText { value: _ } => {
                                // テキストはそのまま追加
                                tokens.push(token.clone());
                            }
                            _ => {
                                // その他のトークン（見出し、リストなど）はそのまま追加
                                tokens.push(token.clone());
                            }
                        }
                    }
                }
            }

            tokens.push(Token::BlockquoteClose);
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

        if input.starts_with("---") {
            tokens.push(Token::ThematicBreak);
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

    /// 引用内のブロックを処理する内部メソッド
    fn process_inner_block(&self, input: &str) -> Vec<Token> {
        self.process_block(input, "")
    }
}
