pub fn spell(n: u64) -> String {
    match n {
        0..=99 => spells_below_100(n),
        100..=999 => spells_hundreds(n),
        _ => spells_bignum(n),
    }
}

pub fn spells_below_100(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "fifteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineeen".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        _ => {
            let r = n % 10;
            format!("{}-{}", spells_below_100(n - r), spells_below_100(r))
        }
    }
}

pub fn spells_hundreds(n: u64) -> String {
    let d = n / 100;
    let r = n % 100;
    let mut s = format!("{} hundred", spells_below_100(d));
    if r != 0 {
        s = format!("{} {}", s, spells_below_100(r));
    }
    s
}

pub fn spells_bignum(n: u64) -> String {
    let mut s = vec![];
    let mut c = vec![0; 7];
    let mut x = n;
    for i in c.iter_mut() {
        let r = x % 1000;
        x = x / 1000;
        *i = r;
    }
    for (i, v) in c.into_iter().enumerate() {
        let name = match i {
            0 => "",
            1 => "thousand",
            2 => "million",
            _ => "too large",
        };
        if v != 0 {
            s.push(format!("{} {}", spell(v), name).trim().to_string());
        }
    }
    s.reverse();
    s.join(" ")
}

