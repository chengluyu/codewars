fn product_fib(p: u64) -> (u64, u64, bool) {
    let mut x: u64 = 0;
    let mut y: u64 = 1;
    while x * y < p {
        let temp = y;
        y = x + y;
        x = temp;
    }
    if x * y == p {
        (x, y, true)
    } else {
        (x, y, false)
    }
}
