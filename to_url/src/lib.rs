pub fn to_url(s: &str) -> String {
    let str : String = s.to_owned();

    str.split_ascii_whitespace().collect::<Vec<&str>>().join("%20")
}



