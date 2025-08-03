pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();

    for c in input.chars() {
        if c.is_ascii_lowercase() {
            let base = 'a' as u8;
            let mut shifted = (c as i8 - base as i8 + key) % 26;
            if shifted < 0 {
                shifted += 26;
            }
            result.push((shifted as u8 + base) as char);
        } else if c.is_ascii_uppercase() {
            let base = 'A' as u8;
            let mut shifted = (c as i8 - base as i8 + key) % 26;
            if shifted < 0 {
                shifted += 26;
            }
            result.push((shifted as u8 + base) as char);
        } else {
            result.push(c);
        }
    }

    result
}

