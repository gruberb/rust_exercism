pub fn series(digits: &str, len: usize) -> Vec<String> {
   let iterations = digits.len() + 1 - len;
   
    (0..iterations)
        .map(|i| digits[0 + i..len + i].to_string())
        .collect::<Vec<(String)>>()
}
