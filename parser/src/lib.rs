mod ast;
mod parser;

pub use ast::*;
pub use parser::Parser;

/// Markdownテキストを解析してASTに変換する関数
pub fn parse(input: &str) -> ast::Root {
    let tokens = lexer::tokenize(input);
    let parser = Parser::new();
    parser.parse(&tokens)
}

#[cfg(test)]
mod tests;
