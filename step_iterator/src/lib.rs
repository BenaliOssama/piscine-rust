pub struct StepIterator<T> {
    start : T,
    end: T,
    step: T,
    done : bool,
}

use std::ops::Add;
impl<T: Copy + Add<Output = T> + PartialOrd + PartialEq> StepIterator<T> {
    pub fn new(start : T, end: T, step: T) -> Self {
        Self {
            start,
            end,
            step,
            done : false,
        }
    }
}

impl<T: PartialEq + Add<Output = T> + PartialOrd + Copy> std::iter::Iterator for StepIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start  > self.end {
            return None;
        }
        let res = self.start ;
        self.start = self.start + self.step;
        Some(res)
    }
}
