pub fn first_subword(mut s: String) -> String {

    let res : Vec<char> = vec![];
    for (i , c) in s.chars().enumerate(){
        if i != 0 && (c.is_uppercase() || c == '_') {
            return s[..i].to_string()
        }
    }
    return s ; 
}
