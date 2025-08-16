#[derive(Debug, Clone)]
pub struct RomanNumber(Vec<char>);

impl RomanNumber {
    pub fn from(mut n: u32) -> Self {
        let mut result = Vec::new();
        let symbols = [
            (1000, 'M'), (900, 'C'), (500, 'D'), (400, 'C'),
            (100, 'C'), (90, 'X'), (50, 'L'), (40, 'X'),
            (10, 'X'), (9, 'I'), (5, 'V'), (4, 'I'), (1, 'I'),
        ];

        for &(value, symbol) in symbols.iter() {
            while n >= value {
                n -= value;
                result.push(symbol);
            }
        }

        RomanNumber(result)
    }

    pub fn to_number(&self) -> u32 {
        let mut n = 0;
        let symbols = vec![('I', 1), ('V', 5), ('X', 10), ('L', 50), ('C', 100), ('D', 500), ('M', 1000)];

        for (i, &c) in self.0.iter().enumerate() {
            let val = symbols.iter().find(|&&(s, _)| s == c).unwrap().1;
            if i + 1 < self.0.len() {
                let next_val = symbols.iter().find(|&&(s, _)| s == self.0[i + 1]).unwrap().1;
                if val < next_val {
                    n -= val;
                } else {
                    n += val;
                }
            } else {
                n += val;
            }
        }

        n
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.to_number() + 1;
        *self = RomanNumber::from(n);
        Some(self.clone())
    }
}
