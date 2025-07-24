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


// the matimatical definition
// fib(0) = 0
// fib(1) = 1
// fib(n) = fib(n - 1) + fib(n - 2)
//
// so matches!(n, 0 | 1) checks if n is 0 or 1
//
// otherwise fibonacci(n-2) + fibonacci(n-1)
// beauty in math and computing.

