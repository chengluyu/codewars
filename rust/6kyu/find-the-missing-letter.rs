// https://www.codewars.com/kata/find-the-missing-letter/train/rust

fn find_missing_letter(chars: &[char]) -> char {
    let mut ans = None;
    for i in 1..chars.len() {
        let (p, v) = (chars[i - 1] as u8, chars[i] as u8);
        if p + 1 != v {
            ans = Some((p + 1) as char);
        }
    }
    ans.unwrap()
}
