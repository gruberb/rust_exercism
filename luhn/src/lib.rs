pub fn is_valid(code: &str) -> bool {
    if code.len() < 2 {
        return false;
    }
    let mut sum = 0;
    let mut valid_chars = 0;
    for (index, c) in code.chars().rev().filter(|c| c != &' ').enumerate() {
        match c.to_digit(10) {
            Some(d) => {
                valid_chars += 1;
                if index % 2 == 1 {
                    let d2 = d * 2;
                    sum += if d2 > 9 { d2 - 9 } else { d2 };
                } else {
                    sum += d;
                }
            }
            None => return false,
        }
    }
    valid_chars > 1 && sum % 10 == 0
}