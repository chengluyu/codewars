fn dbl_linear(n: u32) -> u32 {
    let mut numbers = vec![1u32];
    let mut double = 0usize;
    let mut triple = 0usize;
    for _ in 0..n {
        let two = numbers[double] * 2 + 1;
        let three = numbers[triple] * 3 + 1;
        numbers.push(two.min(three));
        if two <= three {
            double += 1;
        }
        if two >= three {
            triple += 1;
        }
    }
    numbers[n as usize]
}
