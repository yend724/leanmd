use crate::ast::*;
use crate::parse;

#[test]
fn test_parse_paragraph() {
    let input = "This is a paragraph.";
    let ast = parse(input);

    assert_eq!(
        ast,
        Root {
            children: vec![Node::Paragraph {
                children: vec![Node::Text {
                    value: "This is a paragraph.".to_string()
                }]
            }]
        }
    );
}

#[test]
fn test_parse_emphasis() {
    let input = "This is *emphasis* text.";
    let ast = parse(input);

    assert_eq!(
        ast,
        Root {
            children: vec![Node::Paragraph {
                children: vec![
                    Node::Text {
                        value: "This is ".to_string()
                    },
                    Node::Emphasis {
                        children: vec![Node::Text {
                            value: "emphasis".to_string()
                        }]
                    },
                    Node::Text {
                        value: " text.".to_string()
                    }
                ]
            }]
        }
    );
}

#[test]
fn test_parse_strong() {
    let input = "This is **strong** text.";
    let ast = parse(input);

    assert_eq!(
        ast,
        Root {
            children: vec![Node::Paragraph {
                children: vec![
                    Node::Text {
                        value: "This is ".to_string()
                    },
                    Node::Strong {
                        children: vec![Node::Text {
                            value: "strong".to_string()
                        }]
                    },
                    Node::Text {
                        value: " text.".to_string()
                    }
                ]
            }]
        }
    );
}

#[test]
fn test_parse_emphasis_in_strong() {
    let input = "**outer *inner* outer**";
    let ast = parse(input);

    assert_eq!(
        ast,
        Root {
            children: vec![Node::Paragraph {
                children: vec![Node::Strong {
                    children: vec![
                        Node::Text {
                            value: "outer ".to_string()
                        },
                        Node::Emphasis {
                            children: vec![Node::Text {
                                value: "inner".to_string()
                            }]
                        },
                        Node::Text {
                            value: " outer".to_string()
                        }
                    ]
                }]
            }]
        }
    );
}

#[test]
fn test_parse_inline_code() {
    let input = "`code`";
    let ast = parse(input);

    assert_eq!(
        ast,
        Root {
            children: vec![Node::Paragraph {
                children: vec![Node::InlineCode {
                    value: "code".to_string()
                }]
            }]
        }
    );
}

#[test]
fn test_parse_heading() {
    let input = "# Heading 1";
    let ast = parse(input);

    assert_eq!(
        ast,
        Root {
            children: vec![Node::Heading {
                depth: 1,
                children: vec![Node::Text {
                    value: "Heading 1".to_string()
                }]
            }]
        }
    );
}

#[test]
fn test_parse_heading_level2() {
    let input = "## Heading 2";
    let ast = parse(input);

    assert_eq!(
        ast,
        Root {
            children: vec![Node::Heading {
                depth: 2,
                children: vec![Node::Text {
                    value: "Heading 2".to_string()
                }]
            }]
        }
    );
}

#[test]
fn test_parse_code_block() {
    let input = "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```";
    let ast = parse(input);

    assert_eq!(
        ast,
        Root {
            children: vec![Node::Code {
                lang: Some("rust".to_string()),
                meta: None,
                value: "fn main() {\n    println!(\"Hello, world!\");\n}".to_string()
            }]
        }
    );
}

#[test]
fn test_parse_code_block_with_meta() {
    let input = "```rust meta\nfn main() {\n    println!(\"Hello, world!\");\n}\n```";
    let ast = parse(input);

    assert_eq!(
        ast,
        Root {
            children: vec![Node::Code {
                lang: Some("rust".to_string()),
                meta: Some("meta".to_string()),
                value: "fn main() {\n    println!(\"Hello, world!\");\n}".to_string()
            }]
        }
    );
}

#[test]
fn test_parse_blockquote() {
    let input = "> This is a blockquote.";
    let ast = parse(input);

    assert_eq!(
        ast,
        Root {
            children: vec![Node::Blockquote {
                children: vec![Node::Paragraph {
                    children: vec![Node::Text {
                        value: "This is a blockquote.".to_string()
                    }]
                }]
            }]
        }
    );
}

#[test]
fn test_parse_blockquote_with_heading() {
    let input = "> # This is a blockquote.";
    let ast = parse(input);

    assert_eq!(
        ast,
        Root {
            children: vec![Node::Blockquote {
                children: vec![Node::Heading {
                    depth: 1,
                    children: vec![Node::Text {
                        value: "This is a blockquote.".to_string()
                    }]
                }]
            }]
        }
    );
}

#[test]
fn test_parse_unordered_list() {
    let input = "- Item 1\n- Item 2\n- Item 3";
    let ast = parse(input);

    assert_eq!(
        ast,
        Root {
            children: vec![Node::List {
                ordered: false,
                start: None,
                children: vec![
                    Node::ListItem {
                        children: vec![Node::Text {
                            value: "Item 1".to_string()
                        }]
                    },
                    Node::ListItem {
                        children: vec![Node::Text {
                            value: "Item 2".to_string()
                        }]
                    },
                    Node::ListItem {
                        children: vec![Node::Text {
                            value: "Item 3".to_string()
                        }]
                    }
                ]
            }]
        }
    );
}

#[test]
fn test_parse_ordered_list() {
    let input = "1. Item 1\n2. Item 2\n3. Item 3";
    let ast = parse(input);

    assert_eq!(
        ast,
        Root {
            children: vec![Node::List {
                ordered: true,
                start: Some(1),
                children: vec![
                    Node::ListItem {
                        children: vec![Node::Text {
                            value: "Item 1".to_string()
                        }]
                    },
                    Node::ListItem {
                        children: vec![Node::Text {
                            value: "Item 2".to_string()
                        }]
                    },
                    Node::ListItem {
                        children: vec![Node::Text {
                            value: "Item 3".to_string()
                        }]
                    }
                ]
            }]
        }
    );
}

#[test]
fn test_parse_thematic_break() {
    let input = "---";
    let ast = parse(input);

    assert_eq!(
        ast,
        Root {
            children: vec![Node::ThematicBreak]
        }
    );
}

// #[test]
// fn test_parse_link() {
//     let input = "This is a [link](https://example.com) text.";
//     let ast = parse(input);

//     assert_eq!(
//         ast,
//         Root {
//             children: vec![Node::Paragraph {
//                 children: vec![
//                     Node::Text {
//                         value: "This is a ".to_string()
//                     },
//                     Node::Link {
//                         url: "https://example.com".to_string(),
//                         title: None,
//                         children: vec![Node::Text {
//                             value: "link".to_string()
//                         }]
//                     }
//                 ]
//             }]
//         }
//     );
// }

// #[test]
// fn test_parse_image() {
//     let input = "This is an ![image](https://example.com/image.png) text.";
//     let ast = parse(input);

//     let children = match &ast.children[0] {
//         Node::Paragraph(paragraph) => &paragraph.children,
//         _ => panic!("Expected paragraph node"),
//     };

//     let has_image = children.iter().any(|node| {
//         if let Node::Image(image) = node {
//             image.url == "https://example.com/image.png"
//         } else {
//             false
//         }
//     });

//     assert!(
//         has_image,
//         "Expected to find an image node with the correct URL"
//     );
// }

// #[test]
// fn test_parse_complex_document() {
//     let input = "# Markdown Parser\n\nThis is a **Markdown** parser that supports:\n\n- *Emphasis*\n- **Strong**\n- `Code`\n\n```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```\n\n> Blockquotes are also supported.";
//     let ast = parse(input);

//     assert_eq!(ast.children.len(), 5);

//     match &ast.children[0] {
//         Node::Heading(heading) => {
//             assert_eq!(heading.depth, 1);
//             assert_eq!(heading.children.len(), 1);
//         }
//         _ => panic!("Expected heading node"),
//     }

//     match &ast.children[1] {
//         Node::Paragraph(paragraph) => {
//             assert_eq!(paragraph.children.len(), 3);
//         }
//         _ => panic!("Expected paragraph node"),
//     }

//     match &ast.children[2] {
//         Node::List(list) => {
//             assert!(!list.ordered);
//             assert_eq!(list.children.len(), 3);
//         }
//         _ => panic!("Expected list node"),
//     }

//     match &ast.children[3] {
//         Node::Code(code) => {
//             assert_eq!(code.lang, Some("rust".to_string()));
//         }
//         _ => panic!("Expected code node"),
//     }

//     match &ast.children[4] {
//         Node::Blockquote(blockquote) => {
//             assert_eq!(blockquote.children.len(), 1);
//         }
//         _ => panic!("Expected blockquote node"),
//     }
// }
