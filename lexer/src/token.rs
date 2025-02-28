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

    // 強調 (italic)
    EmphasisOpen,
    EmphasisClose,

    // 強調 (bold)
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
    CodeBlockClose,
    CodeBlockText {
        value: String,
    },

    // リスト（順序付き）
    OrderedListOpen {
        start: u32,
    },
    OrderedListClose,

    // リスト（順序なし）
    UnorderedListOpen,
    UnorderedListClose,

    // リストアイテム
    ListItemOpen,
    ListItemClose,

    // ブロック引用
    BlockquoteOpen,
    BlockquoteClose,

    // 水平線
    ThematicBreak,

    // リンク
    LinkOpen {
        url: String,
        title: Option<String>,
    },
    LinkClose,

    // 画像
    Image {
        url: String,
        title: Option<String>,
        alt: Option<String>,
    },

    // tokenize　される前のテキスト
    UnResolvedText {
        value: String,
    },
}
