pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    for c in v.chars(){
        if !c.is_ascii(){
            return false ; 
        }
    }
    return true;
}

pub fn contains(v: &str, pat: &str) -> bool {
    return v.contains(pat);
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[..index], &v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).unwrap()
}
