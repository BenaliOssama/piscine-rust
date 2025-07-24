pub fn fibonacci(n: u32) -> u32 {
    return helper(n, 0 , 1);
}

fn helper(n : u32, current: u32, prev: u32) -> u32{
    if n == 0 {
        return current;
    }
    let temp = current ;
    let next = current + prev ;
    helper(n - 1, next , temp)

}
