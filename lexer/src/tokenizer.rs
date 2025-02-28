use crate::block::BlockProcessor;
use crate::inline::InlineProcessor;
use crate::token::Token;

/// Markdownテキストをトークン化するためのトークナイザー
pub struct Tokenizer {
    block_processor: BlockProcessor,
    inline_processor: InlineProcessor,
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer {
    /// 新しいトークナイザーを作成
    pub fn new() -> Self {
        Self {
            block_processor: BlockProcessor::new(),
            inline_processor: InlineProcessor::new(),
        }
    }

    /// Markdownテキストをトークン化
    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut result_tokens = Vec::new();
        let mut current_scope = String::new();
        let mut current_block_tokens = Vec::new();

        // 一行ずつブロックレベルの解析
        let lines: Vec<&str> = input.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            let block_tokens = self.block_processor.process_block(line, &current_scope);
            let next_scope = self.block_processor.next_block_scope(line, &current_scope);

            current_block_tokens.extend(block_tokens);
            current_scope = next_scope;

            if !current_scope.is_empty() {
                continue;
            }

            // 各ブロックトークン内のインライン要素を解析
            self.process_inline_elements(&mut result_tokens, &current_block_tokens);

            // 最後の行でなければ改行を追加
            if i < lines.len() - 1 {
                result_tokens.push(Token::Newline);
            }

            current_block_tokens.clear();
        }

        // ブロックトークンが残っている場合は閉じる
        if !current_block_tokens.is_empty() {
            self.block_processor.close_tokens(&mut current_block_tokens);
            result_tokens.extend(current_block_tokens);
        }

        result_tokens
    }

    /// ブロックトークン内のインライン要素を処理
    fn process_inline_elements(&self, result_tokens: &mut Vec<Token>, block_tokens: &[Token]) {
        for token in block_tokens {
            match token {
                Token::UnResolvedText { value } => {
                    let inline_tokens = self.inline_processor.process_inline(value);
                    result_tokens.extend(inline_tokens);
                }
                _ => {
                    result_tokens.push(token.clone());
                }
            }
        }
    }
}
