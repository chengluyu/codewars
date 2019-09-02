// https://www.codewars.com/kata/remove-string-spaces/train/rust
fn no_space(x : String) -> String{
  x.chars().filter(|&x| x != ' ').collect()
}
