// https://www.codewars.com/kata/abbreviate-a-two-word-name/train/rust
fn abbrev_name(name: &str) -> String {
    let parts: Vec<&str> = name.split(' ').collect();
    format!("{}.{}", parts[0].chars().next().unwrap(), parts[1].chars().next().unwrap())
}
