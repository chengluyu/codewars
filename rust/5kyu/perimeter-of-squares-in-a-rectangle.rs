// https://www.codewars.com/kata/perimeter-of-squares-in-a-rectangle/train/rust

fn perimeter(n: u64) -> u64 {
    4 * ({
        let mut t = (n + 2, 1u64, 1u64);
        while t.0 > 0 { t = (t.0 - 1, t.2, t.1 + t.2); }
        t.1
    } - 1)
}
