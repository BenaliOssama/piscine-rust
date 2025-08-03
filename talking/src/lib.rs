

pub fn talking(text: &str) -> &str {
    if text.trim().len() == 0 {
        return "Just say something!";
    
    }
    if text.trim().chars().last().unwrap() == '?' {
        if text.chars().all(|c| c.is_uppercase() || !c.is_alphabetic() ) {
            return "Quiet, I am thinking!";
        }else{
            return "Sure.";
        }
    }
    if text.trim().chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
        return "LEAVE ME ALONE!";
    }

    return "Interesting";
}
