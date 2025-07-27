pub fn capitalize_first(input: &str) -> String {
    match input.chars().next(){
        Some(first) => first.to_uppercase().to_string() + &input[1..],
        None => String::new(),
    }
}

pub fn title_case(input: &str) -> String {
    input.split_ascii_whitespace().map(|input| capitalize_first(input)).collect::<Vec<String>>().join(" ")
}

pub fn change_case(input: &str) -> String {
    let mut res : String = String::new();
    for char in input.chars() {
        if char.is_uppercase() {
            res.push_str(&char.to_uppercase().collect::<String>());
        }else if char.is_lowercase(){
            res.push_str(&char.to_lowercase().collect::<String>());
        }else{
            res.push(char);
        }
    }
    res
}


