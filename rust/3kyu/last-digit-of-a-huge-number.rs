fn solve2(x1: u64, x2: u64) -> u64 {
    if x2 == 0 {
        1
    } else {
        (x1 % 10).pow(if x2 % 4 == 0 { 4 } else { x2 % 4 } as u32) % 10
    }
}

fn solve3(x1: u64, x2: u64, x3: u64) -> u64 {
    let pow_x2_x3_equal_0 = x2 == 0 && x3 != 0;
    let pow_x2_x3_mod_4 = match x2 % 4 {
        0 => if x3 == 0 { 1 } else { 0 },
        1 => 1,
        2 => match x3 { 0 => 1, 1 => 2, _ => 0 },
        _ => if x3 % 2 == 1 { 3 } else { 1 }
    };
    // below are same with solve2, but replace x2 with pow(x2, x3)
    if pow_x2_x3_equal_0 {
        1
    } else {
        (x1 % 10).pow(
            if pow_x2_x3_mod_4 == 0 { 4 } else { pow_x2_x3_mod_4 } as u32
        ) % 10
    }
}

fn solve4(x1: u64, x2: u64, x3: u64, x4: u64) -> u64 {
    let pow_x3_x4_equal_0 = x3 == 0 && x4 != 0;
    let pow_x3_x4_is_odd = x3 % 2 == 1 || x4 == 0;
    let pow_x3_x4_is_one = x3 == 1 || x4 == 0;

    // below are same with solve3, but replace x3 with pow(x3, x4)
    let pow_x2_x3_equal_0 = x2 == 0 && !pow_x3_x4_equal_0;
    let pow_x2_x3_mod_4 = match x2 % 4 {
        0 => if pow_x3_x4_equal_0 { 1 } else { 0 },
        1 => 1,
        2 => if pow_x3_x4_equal_0 { 1 } else if pow_x3_x4_is_one { 2 } else { 0 },
        _ => if pow_x3_x4_is_odd { 3 } else { 1 }
    };
    // below are same with solve2, but replace x2 with pow(x2, x3)
    if pow_x2_x3_equal_0 {
        1
    } else {
        (x1 % 10).pow(
            if pow_x2_x3_mod_4 == 0 { 4 }
            else { pow_x2_x3_mod_4 } as u32
        ) % 10
    }
}

fn pow_chain_is_zero(lst: &[u64]) -> bool {
    if lst.len() > 1 {
        lst.first().unwrap().clone() == 0 && !pow_chain_is_zero(&lst[1..])
    } else {
        lst.first().unwrap().clone() == 0
    }
}

fn last_digit(lst: &[u64]) -> u64 {
    match lst.len() {
        0 => 1,
        1 => lst[0] % 10,
        2 => solve2(lst[0], lst[1]),
        3 => solve3(lst[0], lst[1], lst[2]),
        _ => {
            solve4(lst[0], lst[1], lst[2], if pow_chain_is_zero(&lst[3..]) { 0 } else { 1 })
        }
    }
}
