// https://www.codewars.com/kata/expressions-matter/train/rust
fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
    *vec![
        a * b * c,
        a * b + c,
        a * (b + c),
        a + b * c,
        (a + b) * c,
        a + b + c
    ].iter().max().unwrap()
}
