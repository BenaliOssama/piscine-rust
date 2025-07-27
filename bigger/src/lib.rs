use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut biggest  = std::i32::MIN; 
    for (word, num) in h{
        biggest  = i32::max( biggest, num);
    } 
    biggest
}
