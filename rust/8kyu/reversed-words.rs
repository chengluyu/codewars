// I learned `rsplit` and specialized `join` for string vectors.
fn reverse_words(str: &str) -> String {
    str.rsplit(' ').collect::<Vec<&str>>().join(" ")
}
