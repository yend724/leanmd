use crate::ast::*;
use lexer::Token;

/// Markdownトークンを解析してASTに変換するパーサー
pub struct Parser {}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    /// 新しいパーサーを作成
    pub fn new() -> Self {
        Self {}
    }

    /// トークンをASTに変換
    pub fn parse(&self, tokens: &[Token]) -> Root {
        let children = self.parse_nodes(tokens);
        Root { children }
    }

    /// ノードのリストを解析
    #[allow(clippy::only_used_in_recursion)]
    fn parse_nodes(&self, tokens: &[Token]) -> Vec<Node> {
        let mut nodes = Vec::new();

        let mut tokens_iter = tokens.iter().peekable();

        while let Some(&token) = tokens_iter.peek() {
            match token {
                Token::Text { value } => {
                    tokens_iter.next();
                    nodes.push(Node::Text {
                        value: value.to_string(),
                    });
                }
                Token::ParagraphOpen => {
                    tokens_iter.next();
                    let mut acc_tokens = Vec::new();

                    // Token::ParagraphCloseまで
                    while let Some(&token) = tokens_iter.peek() {
                        if matches!(token, Token::ParagraphClose) {
                            break;
                        }
                        acc_tokens.push(token.clone());
                        tokens_iter.next();
                    }

                    nodes.push(Node::Paragraph {
                        children: self.parse_nodes(&acc_tokens),
                    });
                    acc_tokens.clear();
                }
                Token::HeadingOpen { level } => {
                    tokens_iter.next();
                    let mut acc_tokens = Vec::new();

                    while let Some(&token) = tokens_iter.peek() {
                        if matches!(token, Token::HeadingClose) {
                            break;
                        }
                        acc_tokens.push(token.clone());
                        tokens_iter.next();
                    }

                    nodes.push(Node::Heading {
                        depth: *level,
                        children: self.parse_nodes(&acc_tokens),
                    });
                    acc_tokens.clear();
                }
                Token::EmphasisOpen => {
                    tokens_iter.next();
                    let mut acc_tokens = Vec::new();

                    while let Some(&token) = tokens_iter.peek() {
                        if matches!(token, Token::EmphasisClose) {
                            break;
                        }
                        acc_tokens.push(token.clone());
                        tokens_iter.next();
                    }

                    nodes.push(Node::Emphasis {
                        children: self.parse_nodes(&acc_tokens),
                    });
                    acc_tokens.clear();
                }
                Token::StrongOpen => {
                    tokens_iter.next();
                    let mut acc_tokens = Vec::new();

                    while let Some(&token) = tokens_iter.peek() {
                        if matches!(token, Token::StrongClose) {
                            break;
                        }
                        acc_tokens.push(token.clone());
                        tokens_iter.next();
                    }

                    nodes.push(Node::Strong {
                        children: self.parse_nodes(&acc_tokens),
                    });
                    acc_tokens.clear();
                }
                Token::CodeInline { value } => {
                    tokens_iter.next();
                    nodes.push(Node::InlineCode {
                        value: value.to_string(),
                    });
                }
                Token::CodeBlockOpen { lang, meta } => {
                    tokens_iter.next();
                    let mut acc_text = String::new();
                    let mut acc_tokens = Vec::new();

                    while let Some(&token) = tokens_iter.peek() {
                        if let Token::CodeBlockText { value } = token {
                            acc_text.push_str(value);
                            break;
                        }
                        if matches!(token, Token::CodeBlockClose) {
                            break;
                        }
                        acc_tokens.push(token.clone());
                        tokens_iter.next();
                    }

                    nodes.push(Node::Code {
                        lang: lang.clone(),
                        meta: meta.clone(),
                        value: acc_text,
                    });
                    acc_tokens.clear();
                }
                Token::ThematicBreak => {
                    tokens_iter.next();
                    nodes.push(Node::ThematicBreak);
                }
                Token::BlockquoteOpen => {
                    tokens_iter.next();
                    let mut acc_tokens = Vec::new();

                    while let Some(&token) = tokens_iter.peek() {
                        if matches!(token, Token::BlockquoteClose) {
                            break;
                        }
                        acc_tokens.push(token.clone());
                        tokens_iter.next();
                    }

                    nodes.push(Node::Blockquote {
                        children: self.parse_nodes(&acc_tokens),
                    });
                    acc_tokens.clear();
                }
                Token::UnorderedListOpen => {
                    tokens_iter.next();
                    let mut acc_tokens = Vec::new();

                    while let Some(&token) = tokens_iter.peek() {
                        if matches!(token, Token::UnorderedListClose) {
                            break;
                        }
                        acc_tokens.push(token.clone());
                        tokens_iter.next();
                    }

                    nodes.push(Node::List {
                        ordered: false,
                        start: None,
                        children: self.parse_nodes(&acc_tokens),
                    });
                    acc_tokens.clear();
                }
                Token::OrderedListOpen { start } => {
                    tokens_iter.next();
                    let mut acc_tokens = Vec::new();

                    while let Some(&token) = tokens_iter.peek() {
                        if matches!(token, Token::OrderedListClose) {
                            break;
                        }
                        acc_tokens.push(token.clone());
                        tokens_iter.next();
                    }

                    nodes.push(Node::List {
                        ordered: true,
                        start: Some(*start),
                        children: self.parse_nodes(&acc_tokens),
                    });
                    acc_tokens.clear();
                }
                Token::ListItemOpen => {
                    tokens_iter.next();
                    let mut acc_tokens = Vec::new();

                    while let Some(&token) = tokens_iter.peek() {
                        if matches!(token, Token::ListItemClose) {
                            break;
                        }
                        acc_tokens.push(token.clone());
                        tokens_iter.next();
                    }

                    nodes.push(Node::ListItem {
                        children: self.parse_nodes(&acc_tokens),
                    });
                    acc_tokens.clear();
                }
                _ => {
                    tokens_iter.next();
                }
            }
        }

        nodes
    }
}
