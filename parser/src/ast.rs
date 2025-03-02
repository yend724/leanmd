/// Markdownドキュメントのルートノード
#[derive(Debug, Clone, PartialEq)]
pub struct Root {
    /// ルートノードの子ノード
    pub children: Vec<Node>,
}

/// Markdownノードの種類
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    /// 段落
    Paragraph {
        /// 段落の子ノード
        children: Vec<Node>,
    },
    /// 見出し
    Heading {
        /// 見出しレベル（1-6）
        depth: usize,
        /// 見出しの子ノード
        children: Vec<Node>,
    },
    /// テキスト
    Text {
        /// テキスト値
        value: String,
    },
    /// 強調（イタリック）
    Emphasis {
        /// 強調の子ノード
        children: Vec<Node>,
    },
    /// 強調（太字）
    Strong {
        /// 強調の子ノード
        children: Vec<Node>,
    },
    /// インラインコード
    InlineCode {
        /// コード値
        value: String,
    },
    /// コードブロック
    Code {
        /// 言語識別子
        lang: Option<String>,
        /// メタ情報
        meta: Option<String>,
        /// コード値
        value: String,
    },
    /// 引用
    Blockquote {
        /// 引用の子ノード
        children: Vec<Node>,
    },
    /// リスト
    List {
        /// 順序付きリストかどうか
        ordered: bool,
        /// 開始番号（順序付きリストの場合）
        start: Option<u32>,
        /// リストの子ノード
        children: Vec<Node>,
    },
    /// リストアイテム
    ListItem {
        /// リストアイテムの子ノード
        children: Vec<Node>,
    },
    /// 水平線
    ThematicBreak,
    /// リンク
    Link {
        /// リンクURL
        url: String,
        /// リンクタイトル
        title: Option<String>,
        /// リンクの子ノード
        children: Vec<Node>,
    },
    /// 画像
    Image {
        /// 画像URL
        url: String,
        /// 画像タイトル
        title: Option<String>,
        /// 代替テキスト
        alt: Option<String>,
    },
}
