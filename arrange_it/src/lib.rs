pub fn arrange_phrase(phrase: &str) -> String {

    let mut stack : Vec<String> = vec![];
    let fragment : Vec<&str> = phrase.split_whitespace().collect(); 

    while fragment.len() != stack.len() {
        for (_, e) in fragment.iter().enumerate() {

            let index = stack.len() as usize  + 1;
            let index_str = &index.to_string();

            if e.contains(index_str){
                stack.push(e.replace(index_str, ""));
                //fragment.remove(i);
                break;
            }
        }
    } 
    return stack.join(" ");

}
