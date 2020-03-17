fn last_digit(a: &str, b: &str) -> i32 {
    if b == "0" {
        return 1;
    }
    let base = a.chars().last().unwrap().to_digit(10).unwrap();
    let exponent = if b.len() >= 2 {
        b[(b.len() - 2)..].parse::<u32>().unwrap()
    } else {
        b.chars().last().unwrap().to_digit(10).unwrap()
    } % 4;
    base.pow(if exponent == 0 { 4 } else { exponent }) as i32 % 10
}
