pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (i , num) in array.iter().rev().enumerate() {
        if key == *num {
            return Some(i); 
        }
    }
    None
}
