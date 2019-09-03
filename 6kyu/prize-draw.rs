// https://www.codewars.com/kata/5616868c81a0f281e500005c

use std::cmp::Ordering;

fn rank(st: &str, we: Vec<i32>, n: usize) -> &str {
    if st.len() == 0 {
        return "No participants";
    }
    let first_names: Vec<&str> = st.split(",").collect();
    if n > first_names.len() {
        return "Not enough participants";
    }
    first_names.iter().for_each(|x| print!("{}, ", x)); println!();
    let winning_numbers: Vec<i32> = first_names.iter().map(|name| {
        name.len() as u32 + name.chars().fold(0u32, |acc, ch| acc + match ch {
            'A'..='Z' => ch as u32 - 65 + 1,
            'a'..='z' => ch as u32 - 97 + 1,
            _ => 0
        })
    }).zip(we).map(|(som, w)| som as i32 * w).collect();
    winning_numbers.iter().for_each(|x| print!("{}, ", x)); println!();
    let mut pairs: Vec<(&i32, &str)> = winning_numbers.iter().zip(first_names).collect();
    pairs.sort_by(|a, b| {
        match b.0.cmp(&a.0) {
            Ordering::Equal => a.1.cmp(&b.1),
            otherwise => otherwise,
        }
    });
    pairs[n - 1].1
}
