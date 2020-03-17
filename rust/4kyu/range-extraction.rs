mod solution {
    pub fn range_extraction(a: &[i32]) -> String {
        let mut ranges: Vec<(i32, i32)> = vec![];
        for &value in a {
            if ranges.is_empty() {
                ranges.push((value, value));
            } else if ranges.last().unwrap().1 + 1 == value {
                let begin = ranges.pop().unwrap().0;
                ranges.push((begin, value));
            } else {
                ranges.push((value, value));
            }
        }
        ranges
            .iter()
            .map(|(x, y)| if x == y { format!("{}", x) } else { format!("{}{}{}", x, if x + 1 == *y { ',' } else { '-' }, y) })
            .collect::<Vec<String>>()
            .join(",")
    }
}
