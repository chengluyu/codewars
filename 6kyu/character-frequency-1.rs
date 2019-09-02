// https://www.codewars.com/kata/character-frequency-1/train/rust

use std::collections::BTreeMap;

fn letter_frequency(input: &str) -> BTreeMap<char, i32> {
    let mut map: BTreeMap<char, i32> = BTreeMap::new();
    for c in input.chars().filter(|c| c.is_ascii_alphabetic()) {
        map.entry(c.to_ascii_lowercase()).and_modify(|e| *e += 1).or_insert(1);
    }
    map
}
