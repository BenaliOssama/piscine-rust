pub fn talking(text: &str) -> &str {

    let trimmed = text.trim();

    if trimmed.is_empty() {
        return "Just say something!";
    }

    if trimmed.chars().last().unwrap() == '?' {
        if trimmed.chars().all(|c| c.is_uppercase() || !c.is_alphabetic())
            && trimmed.chars().any(|c| c.is_alphabetic()) {
            return "Quiet, I am thinking!";
        } else {
            return "Sure.";
        }
    }

    if trimmed.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
        return "There is no need to yell, calm down!";
    }

    "Interesting"
}

