pub fn number_logic(num: u32) -> bool {
    let str_num = num.to_string();
    let power = str_num.len() as u32;

    let sum = str_num.chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d.pow(power))
        .sum::<u32>();

    num == sum
}

