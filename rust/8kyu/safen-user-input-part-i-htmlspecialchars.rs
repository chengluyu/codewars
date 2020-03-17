fn html_special_chars(html: &str) -> String {
    html.chars().map(|c| match c {
        '<' => "&lt;".to_string(),
        '>' => "&gt;".to_string(),
        '"' => "&quot;".to_string(),
        '&' => "&amp;".to_string(),
        c => c.to_string(),
    }).collect()
}
