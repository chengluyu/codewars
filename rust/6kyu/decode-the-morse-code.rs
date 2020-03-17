// https://www.codewars.com/kata/decode-the-morse-code/train/rust
impl MorseDecoder {
    fn decode_morse(&self, encoded: &str) -> String {
        encoded.split("   ").map(|word| {
            word.split(" ").map(|letter| {
                self.morse_code.get(letter).map(|s| s.as_str()).unwrap_or("")
            }).filter(|s| !s.is_empty()).collect::<Vec<&str>>().concat()
        }).filter(|s| !s.is_empty()).collect::<Vec<String>>().join(" ")
    }
}
