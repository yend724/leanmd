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
            Token::CodeInlineOpen,
            Token::Text {
                value: "code".to_string()
            },
            Token::CodeInlineClose,
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
                value: "code1".to_string()
            },
            Token::CodeBlockText {
                value: "code2".to_string()
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
fn test_blockquote_with_heading_level2_tokens() {
    let input = "> ## Second level heading";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::BlockquoteOpen,
            Token::HeadingOpen { level: 2 },
            Token::Text {
                value: "Second level heading".to_string()
            },
            Token::HeadingClose,
            Token::BlockquoteClose
        ]
    );
}
