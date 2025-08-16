use lalgebra_scalar::Scalar;


use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Clone)]
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

        for h in 0..hight {
            // -> 0 1
            for w in 0..width {
                //-> 0
                result.0[h][w] = self.0[h][w] + m.0[h][w];
            }
        }
        Some(result)
    }
}

impl<T: Scalar<Item=T>> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
		self.0.len()
	}

    pub fn number_of_rows(&self) -> usize {
		return if self.0.len() == 0 {
			0
		}else{
			self.0[0].len()
		}
	}

    pub fn row(&self, n: usize) -> Vec<T> {
		return self.0[n].clone();
	}

    pub fn col(&self, n: usize) -> Vec<T> {
        if self.number_of_rows() < n + 1  {
            return vec![];
        }
        let mut v : Vec<T> = vec![];
        for i in 0..self.number_of_cols(){
            v.push(self.0[i][n])
        }
        return v ;
	}
}

impl<T: Scalar<Item = T >> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn mul(self, mm : Matrix<T>) -> <Self as Add<Matrix<T>>>::Output {
		let n = self.number_of_cols();
		let m = self.number_of_rows();

		let n2 = mm.number_of_cols();
		let p = mm.number_of_rows();

		if n != n2 {
			return None;
		}

		let mut result = Matrix::zero(m, p);

		for i in 0..m{
			for j in 0..p {
				let mut sum  = <T as Scalar>::zero()  ;
				for k in 0..n {
					sum = sum + self.0[i][k] * mm.0[k][j];
				}
				result.0[i][j] = sum;
			}
		}
		return Some(result);

	}
}
// function multiply_matrices(A, B):
//     (m, n) = dimensions of A
//     (n2, p) = dimensions of B
//     if n != n2:
//         error "incompatible sizes"

//     Create C as m × p zero matrix

//     for i from 0 to m-1:          // rows of A
//         for j from 0 to p-1:      // columns of B
//             sum = 0
//             for k from 0 to n-1:  // columns of A / rows of B
//                 sum = sum + A[i][k] * B[k][j]
//             C[i][j] = sum

//     return C