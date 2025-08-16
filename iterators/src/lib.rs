pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v <= 1 {
            return None;
        }
        let num = self.v;
        if num | 1 != num {
            self.v /= 2;
        } else {
            self.v = 3 * num + 1;
        }
        Some(Collatz { v: num })
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Self { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    if n < 2 {
        return 0;
    }
    let mut count = 0;
    let mut n = n;
    while n != 1 {
        if n & 1 == 0{
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        count += 1;
    }
    count
}
