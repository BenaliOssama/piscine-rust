pub fn capitalize_first(input: &str) -> String {
    match input.chars().next(){
        Some(first) => first.to_uppercase().to_string() + &input[1..],
        None => String::new(),
    }
}

pub fn title_case(input: &str) -> String {
    input.split(" ").map(|input| {
        input.split("\t").map(|input| {
            capitalize_first(input)
        }).collect::<Vec<String>>().join("\t")
    }).collect::<Vec<String>>().join(" ")
}

pub fn change_case(input: &str) -> String {
    input.chars().map(|char| {
        if char.is_uppercase() {
            char.to_lowercase().collect::<String>()
        }else if char.is_lowercase(){
            char.to_uppercase().collect::<String>()
        }else{
            char.to_string()
        }
    }).collect()
}


