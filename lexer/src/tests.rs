use crate::token::Token;
use crate::tokenize;

#[test]
fn test_text_token() {
    let input = "text";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "text".to_string()
            },
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_text_multiple_lines() {
    let input = "first line\nsecond line";
    let tokens = tokenize(input);

    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "first line".to_string()
            },
            Token::ParagraphClose,
            Token::Newline,
            Token::ParagraphOpen,
            Token::Text {
                value: "second line".to_string()
            },
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_heading_level_1_token() {
    let input = "# Heading";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::HeadingOpen { level: 1 },
            Token::Text {
                value: "Heading".to_string()
            },
            Token::HeadingClose
        ]
    );
}

#[test]
fn test_heading_level_2_token() {
    let input = "## Heading";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::HeadingOpen { level: 2 },
            Token::Text {
                value: "Heading".to_string()
            },
            Token::HeadingClose
        ]
    );
}

#[test]
fn test_thematic_break_token() {
    let input = "---";
    let tokens = tokenize(input);
    assert_eq!(tokens, vec![Token::ThematicBreak]);
}

#[test]
fn test_emphasis_tokens() {
    let input = "text *emphasis*";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "text ".to_string()
            },
            Token::EmphasisOpen,
            Token::Text {
                value: "emphasis".to_string()
            },
            Token::EmphasisClose,
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_emphasis_unclosed_tokens() {
    let input = "text *emphasis";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "text ".to_string()
            },
            Token::Text {
                value: "*".to_string()
            },
            Token::Text {
                value: "emphasis".to_string()
            },
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_strong_tokens() {
    let input = "text **strong**";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "text ".to_string()
            },
            Token::StrongOpen,
            Token::Text {
                value: "strong".to_string()
            },
            Token::StrongClose,
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_strong_unclosed_tokens() {
    let input = "text **strong";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "text ".to_string()
            },
            Token::Text {
                value: "**".to_string()
            },
            Token::Text {
                value: "strong".to_string()
            },
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_strong_emphasis_nested_tokens() {
    let input = "**outer *inner* outer**";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::StrongOpen,
            Token::Text {
                value: "outer ".to_string()
            },
            Token::EmphasisOpen,
            Token::Text {
                value: "inner".to_string()
            },
            Token::EmphasisClose,
            Token::Text {
                value: " outer".to_string()
            },
            Token::StrongClose,
            Token::ParagraphClose
        ]
    );
}

// 3つ以上の*が連続している場合はテキストとして処理する
#[test]
fn test_three_asterisks() {
    let input = "***asterisks***";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "***".to_string()
            },
            Token::Text {
                value: "asterisks".to_string()
            },
            Token::Text {
                value: "***".to_string()
            },
            Token::ParagraphClose
        ]
    );
}

// インラインコードの処理
#[test]
fn test_code_inline_tokens() {
    let input = "`code`";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::CodeInline {
                value: "code".to_string()
            },
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_code_inline_unclosed_tokens() {
    let input = "`inline code";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "`".to_string()
            },
            Token::Text {
                value: "inline code".to_string()
            },
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_two_backticks_tokens() {
    let input = "``backticks``";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "``".to_string()
            },
            Token::Text {
                value: "backticks".to_string()
            },
            Token::Text {
                value: "``".to_string()
            },
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_code_block_tokens() {
    let input = "```\ncode\n```";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::CodeBlockOpen {
                lang: None,
                meta: None
            },
            Token::CodeBlockText {
                value: "code".to_string()
            },
            Token::CodeBlockClose
        ]
    );
}

#[test]
fn test_code_block_with_lang_tokens() {
    let input = "```rust\ncode\n```";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::CodeBlockOpen {
                lang: Some("rust".to_string()),
                meta: None
            },
            Token::CodeBlockText {
                value: "code".to_string()
            },
            Token::CodeBlockClose
        ]
    );
}

#[test]
fn test_code_block_with_meta_tokens() {
    let input = "```rust meta\ncode\n```";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::CodeBlockOpen {
                lang: Some("rust".to_string()),
                meta: Some("meta".to_string())
            },
            Token::CodeBlockText {
                value: "code".to_string()
            },
            Token::CodeBlockClose
        ]
    );
}

#[test]
fn test_code_block_multiline_tokens() {
    let input = "```\ncode1\ncode2\n```";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::CodeBlockOpen {
                lang: None,
                meta: None
            },
            Token::CodeBlockText {
                value: "code1\ncode2".to_string()
            },
            Token::CodeBlockClose
        ]
    );
}

#[test]
fn test_code_block_unclosed_tokens() {
    let input = "```\ncode";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::CodeBlockOpen {
                lang: None,
                meta: None
            },
            Token::CodeBlockText {
                value: "code".to_string()
            },
            Token::CodeBlockClose
        ]
    );
}

#[test]
fn test_blockquote_tokens() {
    let input = "> quoted text";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::BlockquoteOpen,
            Token::ParagraphOpen,
            Token::Text {
                value: "quoted text".to_string()
            },
            Token::ParagraphClose,
            Token::BlockquoteClose
        ]
    );
}

#[test]
fn test_blockquote_with_formatting_tokens() {
    let input = "> quoted *emphasis* text";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::BlockquoteOpen,
            Token::ParagraphOpen,
            Token::Text {
                value: "quoted ".to_string()
            },
            Token::EmphasisOpen,
            Token::Text {
                value: "emphasis".to_string()
            },
            Token::EmphasisClose,
            Token::Text {
                value: " text".to_string()
            },
            Token::ParagraphClose,
            Token::BlockquoteClose
        ]
    );
}

#[test]
fn test_blockquote_multiline_tokens() {
    let input = "> first line\n> second line";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::BlockquoteOpen,
            Token::ParagraphOpen,
            Token::Text {
                value: "first line".to_string()
            },
            Token::ParagraphClose,
            Token::BlockquoteClose,
            Token::Newline,
            Token::BlockquoteOpen,
            Token::ParagraphOpen,
            Token::Text {
                value: "second line".to_string()
            },
            Token::ParagraphClose,
            Token::BlockquoteClose
        ]
    );
}

#[test]
fn test_blockquote_with_heading_tokens() {
    let input = "> # Heading in blockquote";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::BlockquoteOpen,
            Token::HeadingOpen { level: 1 },
            Token::Text {
                value: "Heading in blockquote".to_string()
            },
            Token::HeadingClose,
            Token::BlockquoteClose
        ]
    );
}

#[test]
fn test_image_tokens() {
    let input = "![alt](image)";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Image {
                url: "image".to_string(),
                title: None,
                alt: Some("alt".to_string())
            },
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_image_in_text_tokens() {
    let input = "This is ![alt](image) text.";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "This is ".to_string()
            },
            Token::Image {
                url: "image".to_string(),
                title: None,
                alt: Some("alt".to_string())
            },
            Token::Text {
                value: " text.".to_string()
            },
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_image_unclose_alt_text_tokens() {
    let input = "![alt";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "!".to_string()
            },
            Token::Text {
                value: "[alt".to_string()
            },
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_image_unopend_url_tokens() {
    let input = "![alt]";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "!".to_string()
            },
            Token::Text {
                value: "[alt]".to_string()
            },
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_image_unclosed_url_tokens() {
    let input = "![alt](image";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "!".to_string()
            },
            Token::Text {
                value: "[alt](image".to_string()
            },
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_link_tokens() {
    let input = "[link](url)";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::LinkOpen {
                url: "url".to_string(),
                title: None
            },
            Token::Text {
                value: "link".to_string()
            },
            Token::LinkClose,
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_link_in_text_tokens() {
    let input = "This is [link](url) text.";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "This is ".to_string()
            },
            Token::LinkOpen {
                url: "url".to_string(),
                title: None
            },
            Token::Text {
                value: "link".to_string()
            },
            Token::LinkClose,
            Token::Text {
                value: " text.".to_string()
            },
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_link_unclosed_text_tokens() {
    let input = "[text";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "[text".to_string()
            },
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_link_unopened_url_tokens() {
    let input = "[text]";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "[text]".to_string()
            },
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_link_unclosed_url_tokens() {
    let input = "[text](image";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::ParagraphOpen,
            Token::Text {
                value: "[text](image".to_string()
            },
            Token::ParagraphClose
        ]
    );
}

#[test]
fn test_unordered_list_tokens() {
    let input = "- Item 1";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::UnorderedListOpen,
            Token::ListItemOpen,
            Token::Text {
                value: "Item 1".to_string()
            },
            Token::ListItemClose,
            Token::UnorderedListClose
        ]
    );
}

#[test]
fn test_unordered_list_multiple_items_tokens() {
    let input = "- Item 1\n- Item 2";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::UnorderedListOpen,
            Token::ListItemOpen,
            Token::Text {
                value: "Item 1".to_string()
            },
            Token::ListItemClose,
            Token::Newline,
            Token::ListItemOpen,
            Token::Text {
                value: "Item 2".to_string()
            },
            Token::ListItemClose,
            Token::UnorderedListClose
        ]
    );
}

#[test]
fn test_unordered_list_with_formatting_tokens() {
    let input = "- Item with *emphasis*";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::UnorderedListOpen,
            Token::ListItemOpen,
            Token::Text {
                value: "Item with ".to_string()
            },
            Token::EmphasisOpen,
            Token::Text {
                value: "emphasis".to_string()
            },
            Token::EmphasisClose,
            Token::ListItemClose,
            Token::UnorderedListClose
        ]
    );
}

#[test]
fn test_ordered_list_tokens() {
    let input = "1. Item 1";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::OrderedListOpen { start: 1 },
            Token::ListItemOpen,
            Token::Text {
                value: "Item 1".to_string()
            },
            Token::ListItemClose,
            Token::OrderedListClose
        ]
    );
}

#[test]
fn test_ordered_list_multiple_items_tokens() {
    let input = "1. Item 1\n2. Item 2";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::OrderedListOpen { start: 1 },
            Token::ListItemOpen,
            Token::Text {
                value: "Item 1".to_string()
            },
            Token::ListItemClose,
            Token::Newline,
            Token::ListItemOpen,
            Token::Text {
                value: "Item 2".to_string()
            },
            Token::ListItemClose,
            Token::OrderedListClose
        ]
    );
}

#[test]
fn test_ordered_list_with_formatting_tokens() {
    let input = "1. Item with **strong**";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::OrderedListOpen { start: 1 },
            Token::ListItemOpen,
            Token::Text {
                value: "Item with ".to_string()
            },
            Token::StrongOpen,
            Token::Text {
                value: "strong".to_string()
            },
            Token::StrongClose,
            Token::ListItemClose,
            Token::OrderedListClose
        ]
    );
}

#[test]
fn test_ordered_list_custom_start_tokens() {
    let input = "3. Item starting from 3";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::OrderedListOpen { start: 3 },
            Token::ListItemOpen,
            Token::Text {
                value: "Item starting from 3".to_string()
            },
            Token::ListItemClose,
            Token::OrderedListClose
        ]
    );
}
