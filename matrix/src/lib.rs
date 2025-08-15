use lalgebra_scalar::Scalar;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![<T as Scalar>::zero()]])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut m = Matrix(vec![vec![<T as Scalar>::zero(); n]; n]);
        for i in 0..n {
            m.0[i][i] = <T as Scalar>::one();
        }
        m
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![<T as Scalar>::zero(); col]; row])
    }
}

impl<T: Scalar> Add for Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, _: Matrix<T>) -> <Self as Add<Matrix<T>>>::Output {
        panic!("no implementation for Matrix + Matrix");
    }
}

impl<T: Scalar> Sub for Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, _: Matrix<T>) -> <Self as Add<Matrix<T>>>::Output {
        panic!("no implementation for Matrix - Matrix");
    }
}

impl<T: Scalar> Div for Matrix<T> {
    type Output = Matrix<T>;
    fn div(self, _: Matrix<T>) -> <Self as Add<Matrix<T>>>::Output {
        panic!("no implementation for Matrix / Matrix");
    }
}

impl<T: Scalar> Mul for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, _: Matrix<T>) -> <Self as Add<Matrix<T>>>::Output {
        panic!("no implementation for Matrix * Matrix");
    }
}

//  `Add`, `Sub`, `Mul`, `Div`
//
//
// trait heloo
// trait world
//
// type humn
//
// imple hello for hum
// gn say_hello -> hello man
//
// impl world for hum
// fn say_hell self -> hello bro
//
// let ghost = humn
//
// ghost.say_hello
//
//
// <ghost as Hello>::say_hello
