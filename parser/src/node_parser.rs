use crate::ast::*;
use lexer::Token;
use std::iter::Peekable;
use std::slice::Iter;

/// 特定のクローズトークンまでのトークンを収集する
pub fn collect_tokens_until<F>(
    tokens_iter: &mut Peekable<Iter<Token>>,
    is_close_token: F,
) -> Vec<Token>
where
    F: Fn(&Token) -> bool,
{
    let mut acc_tokens = Vec::new();

    while let Some(&token) = tokens_iter.peek() {
        if is_close_token(token) {
            break;
        }
        acc_tokens.push(token.clone());
        tokens_iter.next();
    }

    acc_tokens
}

/// クローズトークンをスキップする
pub fn skip_close_token<F>(tokens_iter: &mut Peekable<Iter<Token>>, is_close_token: F)
where
    F: Fn(&Token) -> bool,
{
    if tokens_iter.peek().is_some_and(|t| is_close_token(t)) {
        tokens_iter.next();
    }
}

/// テキストノードを解析
pub fn parse_text(tokens_iter: &mut Peekable<Iter<Token>>, value: &str) -> Node {
    tokens_iter.next();
    Node::Text {
        value: value.to_string(),
    }
}

/// 段落ノードを解析
pub fn parse_paragraph(
    tokens_iter: &mut Peekable<Iter<Token>>,
    parse_nodes: fn(&[Token]) -> Vec<Node>,
) -> Node {
    tokens_iter.next();
    let acc_tokens = collect_tokens_until(tokens_iter, |t| matches!(t, Token::ParagraphClose));
    let children = parse_nodes(&acc_tokens);
    skip_close_token(tokens_iter, |t| matches!(t, Token::ParagraphClose));
    Node::Paragraph { children }
}

/// 見出しノードを解析
pub fn parse_heading(
    tokens_iter: &mut Peekable<Iter<Token>>,
    level: usize,
    parse_nodes: fn(&[Token]) -> Vec<Node>,
) -> Node {
    tokens_iter.next();
    let acc_tokens = collect_tokens_until(tokens_iter, |t| matches!(t, Token::HeadingClose));
    let children = parse_nodes(&acc_tokens);
    skip_close_token(tokens_iter, |t| matches!(t, Token::HeadingClose));
    Node::Heading {
        depth: level,
        children,
    }
}

/// 強調（イタリック）ノードを解析
pub fn parse_emphasis(
    tokens_iter: &mut Peekable<Iter<Token>>,
    parse_nodes: fn(&[Token]) -> Vec<Node>,
) -> Node {
    tokens_iter.next();
    let acc_tokens = collect_tokens_until(tokens_iter, |t| matches!(t, Token::EmphasisClose));
    let children = parse_nodes(&acc_tokens);
    skip_close_token(tokens_iter, |t| matches!(t, Token::EmphasisClose));
    Node::Emphasis { children }
}

/// 強調（太字）ノードを解析
pub fn parse_strong(
    tokens_iter: &mut Peekable<Iter<Token>>,
    parse_nodes: fn(&[Token]) -> Vec<Node>,
) -> Node {
    tokens_iter.next();
    let acc_tokens = collect_tokens_until(tokens_iter, |t| matches!(t, Token::StrongClose));
    let children = parse_nodes(&acc_tokens);
    skip_close_token(tokens_iter, |t| matches!(t, Token::StrongClose));
    Node::Strong { children }
}

/// インラインコードノードを解析
pub fn parse_code_inline(tokens_iter: &mut Peekable<Iter<Token>>, value: &str) -> Node {
    tokens_iter.next();
    Node::InlineCode {
        value: value.to_string(),
    }
}

/// コードブロックノードを解析
pub fn parse_code_block(
    tokens_iter: &mut Peekable<Iter<Token>>,
    lang: &Option<String>,
    meta: &Option<String>,
) -> Node {
    tokens_iter.next();
    let mut acc_text = String::new();

    while let Some(&token) = tokens_iter.peek() {
        if let Token::CodeBlockText { value } = token {
            acc_text.push_str(value);
            tokens_iter.next();
            break;
        }
        if matches!(token, Token::CodeBlockClose) {
            break;
        }
        tokens_iter.next();
    }

    skip_close_token(tokens_iter, |t| matches!(t, Token::CodeBlockClose));

    Node::Code {
        lang: lang.clone(),
        meta: meta.clone(),
        value: acc_text,
    }
}

/// 引用ノードを解析
pub fn parse_blockquote(
    tokens_iter: &mut Peekable<Iter<Token>>,
    parse_nodes: fn(&[Token]) -> Vec<Node>,
) -> Node {
    tokens_iter.next();
    let acc_tokens = collect_tokens_until(tokens_iter, |t| matches!(t, Token::BlockquoteClose));
    let children = parse_nodes(&acc_tokens);
    skip_close_token(tokens_iter, |t| matches!(t, Token::BlockquoteClose));
    Node::Blockquote { children }
}

/// 順序なしリストノードを解析
pub fn parse_unordered_list(
    tokens_iter: &mut Peekable<Iter<Token>>,
    parse_nodes: fn(&[Token]) -> Vec<Node>,
) -> Node {
    tokens_iter.next();
    let acc_tokens = collect_tokens_until(tokens_iter, |t| matches!(t, Token::UnorderedListClose));
    let children = parse_nodes(&acc_tokens);
    skip_close_token(tokens_iter, |t| matches!(t, Token::UnorderedListClose));
    Node::List {
        ordered: false,
        start: None,
        children,
    }
}

/// 順序付きリストノードを解析
pub fn parse_ordered_list(
    tokens_iter: &mut Peekable<Iter<Token>>,
    start: u32,
    parse_nodes: fn(&[Token]) -> Vec<Node>,
) -> Node {
    tokens_iter.next();
    let acc_tokens = collect_tokens_until(tokens_iter, |t| matches!(t, Token::OrderedListClose));
    let children = parse_nodes(&acc_tokens);
    skip_close_token(tokens_iter, |t| matches!(t, Token::OrderedListClose));
    Node::List {
        ordered: true,
        start: Some(start),
        children,
    }
}

/// リストアイテムノードを解析
pub fn parse_list_item(
    tokens_iter: &mut Peekable<Iter<Token>>,
    parse_nodes: fn(&[Token]) -> Vec<Node>,
) -> Node {
    tokens_iter.next();
    let acc_tokens = collect_tokens_until(tokens_iter, |t| matches!(t, Token::ListItemClose));
    let children = parse_nodes(&acc_tokens);
    skip_close_token(tokens_iter, |t| matches!(t, Token::ListItemClose));
    Node::ListItem { children }
}

/// 画像ノードを解析
pub fn parse_image(
    tokens_iter: &mut Peekable<Iter<Token>>,
    url: &str,
    title: &Option<String>,
    alt: &Option<String>,
) -> Node {
    tokens_iter.next();
    Node::Image {
        url: url.to_string(),
        title: title.clone(),
        alt: alt.clone(),
    }
}

/// リンクノードを解析
pub fn parse_link(
    tokens_iter: &mut Peekable<Iter<Token>>,
    url: &str,
    title: &Option<String>,
    parse_nodes: fn(&[Token]) -> Vec<Node>,
) -> Node {
    tokens_iter.next();
    let acc_tokens = collect_tokens_until(tokens_iter, |t| matches!(t, Token::LinkClose));
    let children = parse_nodes(&acc_tokens);
    skip_close_token(tokens_iter, |t| matches!(t, Token::LinkClose));
    Node::Link {
        url: url.to_string(),
        title: title.clone(),
        children,
    }
}
