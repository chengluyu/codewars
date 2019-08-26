fn next_item<T: PartialEq<T> + Clone>(slice: &[T], find: T) -> Option<T> {
    match slice.iter().position(|x| *x == find) {
        Some(i) => if i + 1 == slice.len() { None } else { Some(slice[i + 1].clone()) },
        None => None,
    }
}
