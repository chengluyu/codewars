// https://www.codewars.com/kata/number-of-trailing-zeros-of-n/train/rust

fn zeros(n: u64) -> u64 {
    let mut ans = 0u64;
    let mut d = 5;
    while d < n {
        ans += n / d;
        d *= 5;
    }
    ans
}
