use std::ops::{Add, Mul};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T>(pub Vec<T>);

impl<T> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }
}

impl<T> Vector<T>
where
    T: Add<Output = T> + Mul<Output = T> + Clone + Copy + PartialEq + Eq + std::fmt::Debug,
{
    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut sum = self.0[0] * other.0[0];
        for i in 1..self.0.len() {
            sum = sum + (self.0[i] * other.0[i]);
        }
        Some(sum)
    }
}

impl<T> Add for Vector<T>
where
    T: Add<Output = T> + Clone + Copy + PartialEq + Eq + std::fmt::Debug,
{
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let vec = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| *a + *b)
            .collect();

        Some(Vector(vec))
    }
}

