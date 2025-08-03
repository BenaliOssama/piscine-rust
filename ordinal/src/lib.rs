pub fn num_to_ordinal(x: u32) -> String {
        match x {
           x if  x.to_string().ends_with("1") => x.to_string() + "st",
           x if  x.to_string().ends_with("2") => x.to_string() + "nd",
           x if  x.to_string().ends_with("3") => x.to_string() + "rd",
           _ => x.to_string() + "th",
    }
}


