# Lexer

A Rust library for lexical analysis (tokenization) of Markdown text.

## Supported Syntax

- [x] Paragraphs
- [x] Headings (`#`, `##`, ...)
- [x] Emphasis (`*text*`)
- [x] Bold (`**text**`)
- [x] Inline code (`` `code` ``)
- [x] Code blocks (` ```language meta\ncode``` `)
  - [x] Language specification
  - [x] Meta information
- [x] Blockquotes (`> text`)
- [x] Unordered lists (`- text`)
- [x] Ordered lists (`1. text`)
- [x] Links (`[text](url)`)
- [x] Images (`![text](url)`)
- [x] Horizontal rules (`---`)

## Overview

The lexer module is responsible for breaking down Markdown text into a series of tokens. It analyzes the input text character by character and identifies various Markdown elements based on their syntax patterns.





