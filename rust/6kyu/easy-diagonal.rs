fn diagonal(n: u32, p: u32) -> u64 {
    let mut f = vec![1u64; (n + 1) as usize];
    for i in 1..=p {
        for j in 0..(f.len() - i as usize) {
            f[j + 1] += f[j];
        }
    }
    let mut sum = 0u64;
    for i in 0..=((n - p) as usize) {
        sum += f[i];
    }
    sum
}
