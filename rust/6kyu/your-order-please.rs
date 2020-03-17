// https://www.codewars.com/kata/55c45be3b2079eccff00010f/train/rust

fn order(sentence: &str) -> String {
    let mut words: Vec<(&str, u32)> = sentence.split(" ").map(|s| {
        if let Ok(i) = s.chars().filter(|&c| c.is_digit(10)).collect::<String>().parse::<u32>() {
            (s, i)
        } else {
            (s, 0)
        }
    }).collect();
    words.sort_by(|(_, i), (_, j)| i.cmp(j));
    words.iter().map(|&(s, _)| s).collect::<Vec<&str>>().join(" ")
}
