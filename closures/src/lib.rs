pub fn first_fifty_even_square() -> Vec<i32> {
    let even = |num| num | 1 != num ;
    
    let mut v = vec![];
    let mut i: i32 = 1;
    while v.len() != 50 {
        if even(i) {
            v.push(i.pow(2));
        }
        i += 1;
    }
    v
}