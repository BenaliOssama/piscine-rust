use crate::RomanDigit::*;

#[derive(omanDigit {
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
    fn from(value : u32) -> Self {
        match value {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!("Invalid single RomanDigit from {}", value),
        }
    }
}
impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        let mut digits = Vec::new();

        while num > 0 {
            if num >= 1000 {
                digits.push(M);
                num -= 1000;
            } else if num >= 900 {
                digits.push(C);
                digits.push(M);
                num -= 900;
            } else if num >= 500 {
                digits.push(D);
                num -= 500;
            } else if num >= 400 {
                digits.push(C);
                digits.push(D);
                num -= 400;
            } else if num >= 100 {
                digits.push(C);
                num -= 100;
            } else if num >= 90 {
                digits.push(X);
                digits.push(C);
                num -= 90;
            } else if num >= 50 {
                digits.push(L);
                num -= 50;
            } else if num >= 40 {
                digits.push(X);
                digits.push(L);
                num -= 40;
            } else if num >= 10 {
                digits.push(X);
                num -= 10;
            } else if num == 9 {
                digits.push(I);
                digits.push(X);
                num -= 9;
            } else if num >= 5 {
                digits.push(V);
                num -= 5;
            } else if num == 4 {
                digits.push(I);
                digits.push(V);
                num -= 4;
            } else if num >= 1 {
                digits.push(I);
                num -= 1;
            }
        }

        if digits.is_empty() {
            digits.push(RomanDigit::Nulla);
        }

        RomanNumber(digits)
    }
}
