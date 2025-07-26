pub fn arrange_phrase(phrase: &str) -> String {

    let mut res: Vec<String> = vec![]; 
    let table: Vec<&str> = phrase.split_whitespace().collect();


    while res.len() < table.len() {

        let mut digit = res.len() + 1 ;
        let digit_str = &digit.to_string(); 

        for i in 0..table.len() {
            if table[i].contains(digit_str) {
                res.push(table[i].replace(digit_str , ""));//.to_string());
            }
        }
    }
    res.join(" ")
}
