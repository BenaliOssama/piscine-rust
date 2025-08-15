use lalgebra_scalar::Scalar;
use lalgebra_vector::Vector;

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


impl<T: Scalar<Item = T>> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn add(self, m: Matrix<T>) -> <Self as Add<Matrix<T>>>::Output {
        let hight = self.0.len();
        let width = if hight != 0 { self.0[0].len() } else { 0 };

        let hight1 = m.0.len();
        let width1 = if hight != 0 { m.0[0].len() } else { 0 };


        if hight != hight1 || width != width1 {
            return None;
        }

        let mut result = Matrix::zero(width, hight);

        for h in 0..hight { // -> 0 1 
            for w in 0..width {  //-> 0
                result.0[h][w] = self.0[h][w] + m.0[h][w];
            }
        }
        Some(result)
    }
}

impl<T: Scalar<Item = T >> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn sub(self, m: Matrix<T>) -> <Self as Sub<Matrix<T>>>::Output {
        let hight = self.0.len();
        let width = if hight != 0 { self.0[0].len() } else { 0 };

        let hight1 = m.0.len();
        let width1 = if hight != 0 { m.0[0].len() } else { 0 };


        if hight != hight1 || width != width1 {
            return None;
        }

        let mut result = Matrix::zero(width, hight);

        for h in 0..hight { // -> 0 1 
            for w in 0..width {  //-> 0
                result.0[h][w] = self.0[h][w] - m.0[h][w];
            }
        }
        Some(result)
    }
}

// impl<T: Scalar> Div for Matrix<T> {
//     type Output = Matrix<T>;
//     fn div(self, m: Matrix<T>) -> <Self as Add<Matrix<T>>>::Output {
//         panic!("no implementation for Matrix / Matrix");
//     }
// }

// impl<T: Scalar> Mul for Matrix<T> {
//     type Output = Matrix<T>;
//     fn mul(self, m: Matrix<T>) -> <Self as Add<Matrix<T>>>::Output {
//         panic!("no implementation for Matrix * Matrix");
//     }
// }
