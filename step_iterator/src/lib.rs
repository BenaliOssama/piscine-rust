pub struct StepIterator<T> {
    start : T,
    end: T,
    step: T,
}

use std::ops::Add;
impl<T: Copy + Add<Output = T> + PartialOrd + PartialEq> StepIterator<T> {
    pub fn new(start : T, end: T, step: T) -> Self {
        Self {
            start,
            end,
            step,
        }
    }
}

impl<T: PartialEq + Add<Output = T> + PartialOrd + Copy> std::iter::Iterator for StepIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start + self.step > self.end {
            return None;
        }
        self.start = self.start + self.step;
        Some(self.start)
    }
}
