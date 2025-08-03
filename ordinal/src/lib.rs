// it is not about how much you can bim !
// it is about how much you can get Oh snap! 
// and keep moving forward

pub fn num_to_ordinal(x: u32) -> String {
    let last_two = x % 100 ; 

    if last_two >= 11 && last_two < 13 {
        format!("{}th", x)
    }else{
        match x % 10 {
            1 => format!("{}st", x),
            2 => format!("{}nd", x),
            3 => format!("{}rd", x),
            _ => format!("{}th", x),
        }
    }
}


