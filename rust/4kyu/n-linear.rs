// Althought the instruction is terrifying, the test data is so simple.

fn n_linear(ms: &[u32], n: usize) -> u32 {
    let mut numbers = vec![1u32];
    let mut indices: Vec<usize> = ms.iter().map(|_| 0).collect();
    numbers.reserve(n as usize);
    for _ in 0..n {
        let values: Vec<u32> = indices.iter().zip(ms).map(|(&index, &m)| numbers[index] * m + 1).collect();
        let minimal = values.iter().min().unwrap().clone();
        numbers.push(minimal);
        for (i, &value) in values.iter().enumerate() {
            if value == minimal {
                indices[i] += 1
            }
        }
    }
    numbers[n]
}
