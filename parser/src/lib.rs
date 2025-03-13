mod ast;
mod node_parser;
mod parser;

pub use ast::*;
pub use parser::parse;

/// Markdownテキストを解析してASTに変換する関数
pub fn parse_to_ast(input: &str) -> ast::Root {
    let tokens = lexer::tokenize(input);
    parser::parse(&tokens)
}

#[cfg(test)]
mod tests;
