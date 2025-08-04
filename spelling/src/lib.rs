pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let below_20 = [
        "", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen",
        "fifteen", "sixteen", "seventeen",
        "eighteen", "nineteen",
    ];

    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty",
        "sixty", "seventy", "eighty", "ninety",
    ];

    
    fn less_than_20(n: u64, below_20: &[&str]) -> String {
        below_20[n as usize].to_string()
    }

    fn less_than_100(n: u64, below_20: &[&str], tens: &[&str]) -> String {
        if n < 20 {
            less_than_20(n, below_20)
        } else {
            let ten = n / 10;
            let rest = n % 10;
            if rest == 0 {
                tens[ten as usize].to_string()
            } else {
                format!("{}-{}", tens[ten as usize], less_than_20(rest, below_20))
            }
        }
    }

    fn less_than_1000(n: u64, below_20: &[&str], tens: &[&str]) -> String {
        let hundred = n / 100;
        let rest = n % 100;
        if rest == 0 {
            format!("{} hundred", less_than_20(hundred, below_20))
        } else {
            format!("{} hundred {}", less_than_20(hundred, below_20), less_than_100(rest, below_20, tens))
        }
    }


    fn less_than_million(n: u64, below_20: &[&str], tens: &[&str]) -> String {
        let thousand = n / 1000;
        let rest = n % 1000;
        if rest == 0 {
            format!("{} thousand", less_than_1000(thousand, below_20, tens))
        } else {
            format!("{} thousand {}", less_than_1000(thousand, below_20, tens), less_than_1000(rest, below_20, tens))
        }
    }

    if n < 20 {
        less_than_20(n, &below_20)
    } else if n < 100 {
        less_than_100(n, &below_20, &tens)
    } else if n < 1000 {
        less_than_1000(n, &below_20, &tens)
    } else if n < 1_000_000 {
        less_than_million(n, &below_20, &tens)
    } else {
        "one million".to_string()
    }


}

