pub fn to_url(s: &str) -> String {
    let str : String = s.to_owned();

    str.replace(" ", "%20")
}



