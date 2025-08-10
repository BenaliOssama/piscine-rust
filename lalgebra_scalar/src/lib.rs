use std::ops::{Add, Sub, Mul, Div};

pub trait Scalar: 
    Add<Output= Self> +
    Sub<Output= Self> + 
    Mul<Output= Self> +
    Div<Output= Self> +
    Copy + Sized
{
        type Item;
	fn zero() -> Self::Item;
	fn one() -> Self::Item;
}

impl Scalar for u32 {
    type Item = u32;
    fn zero()-> Self::Item{
        0 
    }

    fn one() -> Self::Item{
        1 
    }
}
impl Scalar for  u64{
    type Item = u64;
    fn zero()-> Self::Item{
        0 
    }

    fn one() -> Self::Item{
        1 
    }

}
impl Scalar for i32{
    type Item = i32;
    fn zero()-> Self::Item{
        0 
    }

    fn one() -> Self::Item{
        1 
    }

}
impl Scalar for i64{
    type Item = i64;
    fn zero()-> Self::Item{
        0
    }

    fn one() -> Self::Item{
        1 
    }

}
impl Scalar for f64 {
    type Item = u64;
    fn zero()-> Self::Item{
        0 
    }

    fn one() -> Self::Item{
        1 
    }

}
