use crate::ast::*;
use crate::node_parser;
use lexer::Token;

/// トークンをASTに変換
pub fn parse(tokens: &[Token]) -> Root {
    let children = parse_nodes(tokens);
    Root { children }
}

/// ノードのリストを解析
#[allow(clippy::only_used_in_recursion)]
fn parse_nodes(tokens: &[Token]) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut tokens_iter = tokens.iter().peekable();

    while let Some(&token) = tokens_iter.peek() {
        match token {
            Token::Text { value } => {
                nodes.push(node_parser::parse_text(&mut tokens_iter, value));
            }
            Token::ParagraphOpen => {
                nodes.push(node_parser::parse_paragraph(&mut tokens_iter, parse_nodes));
            }
            Token::HeadingOpen { level } => {
                nodes.push(node_parser::parse_heading(
                    &mut tokens_iter,
                    *level,
                    parse_nodes,
                ));
            }
            Token::EmphasisOpen => {
                nodes.push(node_parser::parse_emphasis(&mut tokens_iter, parse_nodes));
            }
            Token::StrongOpen => {
                nodes.push(node_parser::parse_strong(&mut tokens_iter, parse_nodes));
            }
            Token::CodeInline { value } => {
                nodes.push(node_parser::parse_code_inline(&mut tokens_iter, value));
            }
            Token::CodeBlockOpen { lang, meta } => {
                nodes.push(node_parser::parse_code_block(&mut tokens_iter, lang, meta));
            }
            Token::ThematicBreak => {
                tokens_iter.next();
                nodes.push(Node::ThematicBreak);
            }
            Token::BlockquoteOpen => {
                nodes.push(node_parser::parse_blockquote(&mut tokens_iter, parse_nodes));
            }
            Token::UnorderedListOpen => {
                nodes.push(node_parser::parse_unordered_list(
                    &mut tokens_iter,
                    parse_nodes,
                ));
            }
            Token::OrderedListOpen { start } => {
                nodes.push(node_parser::parse_ordered_list(
                    &mut tokens_iter,
                    *start,
                    parse_nodes,
                ));
            }
            Token::ListItemOpen => {
                nodes.push(node_parser::parse_list_item(&mut tokens_iter, parse_nodes));
            }
            Token::Image { url, title, alt } => {
                nodes.push(node_parser::parse_image(&mut tokens_iter, url, title, alt));
            }
            Token::LinkOpen { url, title } => {
                nodes.push(node_parser::parse_link(
                    &mut tokens_iter,
                    url,
                    title,
                    parse_nodes,
                ));
            }
            Token::Newline => {
                tokens_iter.next();
                nodes.push(Node::Break);
            }
            _ => {
                tokens_iter.next();
            }
        }
    }

    nodes
}
