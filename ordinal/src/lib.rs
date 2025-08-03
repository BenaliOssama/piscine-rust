// it is not about how much you can bim !
// it is about how much you can get Oh snap! 
// and keep moving forward

pub fn num_to_ordinal(x: u32) -> String {
        match x {
           x if  x.to_string().ends_with("11") && x.to_string().len() > 2 => x.to_string() + "st",
           x if  x.to_string().ends_with("12") && x.to_string().len() > 2=> x.to_string() + "nd",
           x if  x.to_string().ends_with("13") && x.to_string().len() > 2=> x.to_string() + "rd",

           x if  x == 11  => x.to_string() + "th",
           x if  x == 12  => x.to_string() + "th",
           x if  x == 13  => x.to_string() + "th",

           x if  x.to_string().ends_with("1") => x.to_string() + "st",
           x if  x.to_string().ends_with("2") => x.to_string() + "nd",
           x if  x.to_string().ends_with("3") => x.to_string() + "rd",
           _ => x.to_string() + "th",
    }
}


