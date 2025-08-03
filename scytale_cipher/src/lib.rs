pub fn scytale_cipher(s: &str, i: u32) -> String {
    if i  == 1 || i as usize >= s.chars().count() {
        return s.to_string();
    }

    let rows = (s.chars().count() as f64 / i as f64).ceil() as usize;
    let mut array = vec![vec![' '; rows]; i as usize];


    // this is smart, fill vertically then just flattern :)
    for (place, char) in s.chars().enumerate() {
        let col = place % i as usize;
        let row = place / i as usize;

        array[col][row] = char;
    }

    array.iter().flatten().collect::<String>().trim_end().to_string()
}


