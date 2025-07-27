pub fn sum(a: &[i32]) -> i32 {
    a.iter().sum()
    //a.iter().copied().reduce(|x, y| x + y).unwrap()
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}
