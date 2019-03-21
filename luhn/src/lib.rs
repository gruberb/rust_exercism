/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut vec: Vec<i32> = Vec::new();
    if code.len() > 1 {
        for x in code.chars() {
            vec.push(x.parse().unwrap());
        }

        for x in vec.iter().step_by(2) {

        }
    } else {
        false
    }
}
