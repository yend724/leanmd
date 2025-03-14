/// 連続する特定の文字をカウントする関数
pub fn count_target_chars(target: char, chars: &[char], start: usize) -> (usize, usize) {
    let mut i = start;
    let len = chars.len();

    // 連続する文字の数をカウント
    while i < len && chars[i] == target {
        i += 1;
    }

    let count = i - start;
    (count, i) // カウント数と新しい位置を返す
}

/// 順序ありリストアイテムかどうかを判定する関数
pub fn is_ordered_list_item(input: &str) -> bool {
    // 数字で始まり、その後に「. 」が続く場合に順序ありリストと判定
    let mut chars = input.chars();
    let mut has_digit = false;

    // 先頭の数字を確認
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            has_digit = true;
        } else if c == '.' {
            // 数字の後にピリオドがある
            if has_digit && chars.next().is_some_and(|next| next.is_whitespace()) {
                return true;
            }
            return false;
        } else {
            return false;
        }
    }

    false
}

/// 順序ありリストアイテムから開始番号とコンテンツを抽出する関数
pub fn extract_ordered_list_item(input: &str) -> (u32, &str) {
    let mut number_end = 0;
    let mut start = 1;

    // 数字部分を抽出
    for (i, c) in input.char_indices() {
        if c.is_ascii_digit() {
            continue;
        } else if c == '.' {
            number_end = i;
            if let Ok(num) = input[..number_end].parse::<u32>() {
                start = num;
            }
            break;
        } else {
            break;
        }
    }

    // コンテンツ部分を抽出（「. 」の後）
    if number_end > 0 && number_end + 2 <= input.len() {
        let content = input[number_end + 2..].trim_start();
        (start, content)
    } else {
        (start, "")
    }
}
