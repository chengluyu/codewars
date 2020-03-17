// https://www.codewars.com/kata/55983863da40caa2c900004e
fn next_bigger_number(n: i64) -> i64 {
    let mut chars: Vec<char> = n.to_string().chars().collect();
    let mut end = chars.len();
    let mut begin = end - 1;
    while begin > 0 && chars[begin - 1] >= chars[begin] {
        begin -= 1;
    }
    if begin == 0 {
        -1i64
    } else {
        let a = begin - 1;
        let mut b = end - 1;
        while chars[b] <= chars[a] {
            b -= 1;
        }
        chars.swap(a, b);
        for i in 0..((end - begin) / 2) {
            chars.swap(begin + i, end - 1 - i);
        }
        chars.iter().fold(
            0i64,
            |acc, &digit| acc * 10 + digit.to_digit(10).unwrap() as i64
        )
    }
}
