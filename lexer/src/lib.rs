mod block;
mod inline;
mod token;
mod tokenizer;

pub use token::Token;
pub use tokenizer::Tokenizer;

/// Markdownテキストをトークン化する関数
pub fn tokenize(input: &str) -> Vec<Token> {
    let tokenizer = Tokenizer::new();
    tokenizer.tokenize(input)
}

#[cfg(test)]
mod tests;
