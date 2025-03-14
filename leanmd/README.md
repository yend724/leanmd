# LeanMD

A simple Markdown parser written in Rust with WebAssembly support.

## Overview

LeanMD is a simple and efficient Markdown parser that converts Markdown text into a structured JSON format. It is built with Rust and compiled to WebAssembly, making it usable in web browsers and Node.js environments.

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

## Usage

### TypeScript

```typescript
import { markdownToJSON } from 'leanmd';

const markdown = '# Hello World';
const json = markdownToJSON(markdown);
console.log(JSON.parse(json));
```

## API

- `markdownToJSON(input: string): string` - Converts Markdown text to a JSON string
- `markdownToJSONPretty(input: string): string` - Converts Markdown text to a prettified JSON string

## License

[MIT](../LICENSE)