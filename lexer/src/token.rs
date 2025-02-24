#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // 改行
    Newline,

    // テキスト
    Text {
        value: String,
    },

    // 見出し（行頭の '#' をレベル付きで）
    HeadingOpen {
        level: usize,
    },
    HeadingClose,

    // 段落
    ParagraphOpen,
    ParagraphClose,

    // インライン要素（強調／太字）
    EmphasisOpen,
    EmphasisClose,
    StrongOpen,
    StrongClose,

    // コードインライン
    CodeInlineOpen,
    CodeInlineClose,

    // コードブロック
    CodeBlockOpen {
        lang: Option<String>,
        meta: Option<String>,
    },
    CodeBlockText {
        value: String,
    },
    CodeBlockClose,

    // リスト（順序付き／順序なし）
    OrderedList {
        number: u32,
    },
    UnorderedList,

    // ブロック引用
    Blockquote,

    // 水平線
    ThematicBreak,

    // リンク open/close トークン
    LinkTextOpen,
    LinkTextClose,
    LinkUrlOpen,
    LinkUrlClose,

    // 画像 open/close トークン
    ImageAltOpen,
    ImageAltClose,
    ImageUrlOpen,
    ImageUrlClose,

    // tokenize　される前のテキスト
    UnResolvedText {
        value: String,
    },
}
