open Batteries;;

let is_palindrome (s: string): bool =
  let t = BatString.lowercase s in
    t = BatString.rev t
