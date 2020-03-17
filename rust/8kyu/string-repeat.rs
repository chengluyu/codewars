fn repeat_str(src: &str, count: usize) -> String {
    let mut buf = String::new();
    for _ in 0..count { buf.push_str(src); }
    buf
}
