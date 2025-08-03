pub fn is_pangram(s: &str) -> bool {
    for c in ('a' as u8 ..= 'z' as u8) {
        if ! s.to_lowercase().contains(c as char){
            return false;
        }
    }
    true
}
