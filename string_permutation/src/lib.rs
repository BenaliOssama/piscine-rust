use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
    
        return false;
    }
    let mut freq : HashMap<char, isize> = HashMap::new();

    for char in s1.chars() {
        match freq.get(&char) {
            Some(num) => {
                freq.insert(char, num + 1 );
            },
             _ => {freq.insert(char, 1) ;} ,
        }
    }

    
    for char in s2.chars(){
        match freq.get(&char) {
            Some(num) => {
                freq.insert(char, num - 1);
            },
            None => {
                ()
            }
        }
    }

    for (word , count ) in freq {
        if count != 0 {
            return false;
        }
    }
    true
}
