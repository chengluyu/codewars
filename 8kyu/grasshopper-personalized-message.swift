// https://www.codewars.com/kata/grasshopper-personalized-message/train/swift

func great(_ name: String, _ owner: String) -> String {
  if name == owner {
    return "Hello boss"
  }
  return "Hello guest"
}
