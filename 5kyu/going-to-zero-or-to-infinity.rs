fn going(n: i32) -> f64 {
    let mut result: f64 = 0f64;
    let mut acc: f64 = 1f64;
    for i in (1..=n).rev() {
        result += acc;
        acc /= i as f64;
    }
    format!("{:.6}", result).parse().unwrap()
}
