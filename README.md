# LeanMD

A simple Markdown parser written in Rust with WebAssembly support.

## Overview

LeanMD is a simple Markdown parser that converts Markdown text into a structured JSON format. It is built with Rust and compiled to WebAssembly, making it usable in web browsers and Node.js environments.

## Project Structure

The project is organized into three main components:

1. **lexer** - Tokenizes Markdown text into a sequence of tokens
2. **parser** - Converts tokens into an Abstract Syntax Tree (AST)
3. **leanmd** - Main library that provides the public API and WebAssembly bindings

## Supported Syntax

- Paragraphs
- Headings (`#`, `##`, ...)
- Emphasis (`*text*`)
- Bold (`**text**`)
- Inline code (`` `code` ``)
- Code blocks (` ```language meta\ncode``` `)
  - Language specification
  - Meta information
- Blockquotes (`> text`)
- Unordered lists (`- text`)
- Ordered lists (`1. text`)
- Links (`[text](url)`)
- Images (`![text](url)`)
- Horizontal rules (`---`)

## License

[MIT](./LICENSE)

## Author

[YEND](https://github.com/yend724)