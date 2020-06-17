fn next_id(ids: &[usize]) -> usize {
    for id in 0..ids.len() {
        if let None = ids.iter().find(|&&x| x == id) {
            return id
        }
    }
    ids.len()
}
