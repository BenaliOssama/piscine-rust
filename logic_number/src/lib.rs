pub fn number_logic(num: u32) -> bool {
    let str_num = num.to_string();
    let power = str_num.len() as u32;

    let sum  = str_num.chars().map(|c| c.to_string().parse::<u32>().unwrap()).reduce(|sum, y | {
        sum + y.pow(power)
    }).unwrap();
    

    num == sum
}
