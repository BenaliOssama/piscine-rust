// roman_numbers_iter/src/lib.rs

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        match n {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => panic!("invalid roman digit {}", n),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut n: u32) -> Self {
        if n == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        let mut res = Vec::new();
        let table = [
            (1000, vec![RomanDigit::M]),
            (900, vec![RomanDigit::C, RomanDigit::M]),
            (500, vec![RomanDigit::D]),
            (400, vec![RomanDigit::C, RomanDigit::D]),
            (100, vec![RomanDigit::C]),
            (90, vec![RomanDigit::X, RomanDigit::C]),
            (50, vec![RomanDigit::L]),
            (40, vec![RomanDigit::X, RomanDigit::L]),
            (10, vec![RomanDigit::X]),
            (9, vec![RomanDigit::I, RomanDigit::X]),
            (5, vec![RomanDigit::V]),
            (4, vec![RomanDigit::I, RomanDigit::V]),
            (1, vec![RomanDigit::I]),
        ];

        for (v, digits) in table.iter() {
            res.extend_from_slice(&digits.repeat((n / *v) as usize));
            n %= *v;
        }

        RomanNumber(res)
    }
}

impl From<RomanNumber> for u32 {
    fn from(n: RomanNumber) -> Self {
        let value = |d| {
            match d {
                RomanDigit::I => 1,
                RomanDigit::V => 5,
                RomanDigit::X => 10,
                RomanDigit::L => 50,
                RomanDigit::C => 100,
                RomanDigit::D => 500,
                RomanDigit::M => 1000,
                RomanDigit::Nulla => 0,
            }
        };

        let mut res = 0;
        let mut i = 0;
        let digits = &n.0;

        while i < digits.len() {
            let curr = value(digits[i]);
            let next = if i + 1 < digits.len() { value(digits[i + 1]) } else { 0 };

            if curr < next {
                res += next - curr;
                i += 2;
            } else {
                res += curr;
                i += 1;
            }
        }

        res
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let n = u32::from(self.clone()) + 1;
        *self = RomanNumber::from(n);
        Some(self.clone())
    }
}
