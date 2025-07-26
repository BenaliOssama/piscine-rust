
pub fn do_operations(v: &mut [String]) {
    // v slice of 
    v.iter_mut().for_each(|num| {
        if num.contains("+"){
            *num = num.split("+").map(|num| {
                num.parse::<i32>().unwrap()
            }).reduce(|x, y| x + y).unwrap().to_string();
        }else if num.contains("-"){
            *num = num.split("-").map(|num| {
                num.parse::<i32>().unwrap()
            }).reduce(|x, y| x - y).unwrap().to_string();
        }
    });
}


pub fn delete_and_backspace(s: &mut String) {
    let copy = s.clone();
    s.clear();
    let mut stack: Vec<char> = vec![];
    let mut skip : usize = 0 ; 

    for char in copy.chars() {
        if char == '-' {
            stack.pop();
        }else if char == '+' {
           skip += 1 ; 
        }else {
            if skip > 0 {
                skip -= 1 ;
                continue
            }
            stack.push(char);
        }

    }
    s.extend(stack.iter()) ;
}

