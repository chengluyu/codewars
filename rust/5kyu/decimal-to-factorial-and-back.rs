// https://www.codewars.com/kata/54e320dcebe1e583250008fd

use std::char;

const fact_base: [u64; 21] = [
    1u64,
    1u64,
    2u64,
    6u64,
    24u64,
    120u64,
    720u64,
    5040u64,
    40320u64,
    362880u64,
    3628800u64,
    39916800u64,
    479001600u64,
    6227020800u64,
    87178291200u64,
    1307674368000u64,
    20922789888000u64,
    355687428096000u64,
    6402373705728000u64,
    121645100408832000u64,
    2432902008176640000u64,
];

fn dec2_fact_string(nb: u64) -> String {
    let mut i: usize = 0;
    while i < fact_base.len() && fact_base[i + 1] < nb {
        i += 1;
    }
    let mut n = nb;
    let mut buf = String::new();
    loop {
        buf.push(char::from_digit((n / fact_base[i]) as u32, 36).unwrap().to_ascii_uppercase());
        n %= fact_base[i];
        if i == 0 {
            break;
        } else {
            i -= 1;
        }
    }
    buf
}

fn fact_string_2dec(s: String) -> u64 {
    s.chars().enumerate().map(|(i, c)| (s.len() - i - 1, c.to_digit(36).unwrap() as u64))
        .fold(0u64, |acc, (ind, val)| acc + fact_base[ind] * val)
}
