fn create_phone_number(x: &[u8]) -> String {
    format!("({}{}{}) {}{}{}-{}{}{}{}", x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7], x[8], x[9])
}
