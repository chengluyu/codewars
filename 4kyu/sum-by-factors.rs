// https://www.codewars.com/kata/sum-by-factors/train/rust

use std::collections::BTreeMap;

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let mut counter: BTreeMap<i64, i64> = BTreeMap::new();
    for x in l {
        let mut n = x;
        let mut factor = 2i64;
        while factor <= n.abs() {
            let mut not_added = true;
            while n % factor == 0 {
                if not_added {
                    counter.entry(factor).and_modify(|e| *e += x).or_insert(x);
                    not_added = false;
                }
                n /= factor;
            }
            factor += 1;
        }
    }
    counter.into_iter().collect()
}
