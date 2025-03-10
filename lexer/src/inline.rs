fn process_emphasis(input: &str) -> Vec<Token> {
    let graphemes = input.graphemes(true).collect::<Vec<&str>>();
    let mut result_tokens = Vec::new();
    let mut acc_text = String::new();
}
