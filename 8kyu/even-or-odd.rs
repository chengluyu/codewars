// https://www.codewars.com/kata/even-or-odd/train/rust

fn even_or_odd(i: i32) -> &'static str {
    if i % 2 == 0 { "Even" } else { "Odd" }
}
