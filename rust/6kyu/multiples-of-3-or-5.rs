fn p(n: i32) -> i32 { n * (n + 1) / 2 }

fn solution(m: i32) -> i32 {
    let n = m - 1;
    3 * p(n / 3) + 5 * p(n / 5) - 15 * p(n / 15)
}
