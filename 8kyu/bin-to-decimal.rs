// I learned `T::from_str_radix` where `T` is any numeric type.
fn bin_to_decimal(inp: &str) -> i32 {
    i32::from_str_radix(inp, 2).unwrap()
}
