// This drove me to think: what is the underlying type of `char`?
fn get_char(c: i32) -> char {
    std::char::from_u32(c as u32).unwrap()
    // Alternative solution: c as u8 as char
}
