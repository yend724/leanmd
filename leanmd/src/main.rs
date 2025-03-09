fn main() {
    let markdown = r#"
# Hello, leanmd!

This is a **Markdown** parser with *JSON* support.

## Features

- Parses Markdown
- Converts to JSON
- Easy to use

```rust
fn example() {
    println!("Hello, world!");
}
```

[Visit our website](https://example.com)
"#;

    println!("Markdown input:\n{}\n", markdown);
    println!("JSON output:\n{}", leanmd::parse_markdown(markdown));
}
