pub fn first_fifty_even_square() -> Vec<i32> {
    let even = |num| num | 1 != num ;
    
    let mut v = vec![];
    let mut i = 0;
    while v.len() != 50 {
        if even(i) {
            v.push(i);
        }
        i += 1;
    }
    v
}