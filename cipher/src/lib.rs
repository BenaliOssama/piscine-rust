#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    if original.len() == 0 && ciphered.len() == 0 {
        return Ok(());
    }
    // get the original from the cipher
    let mut cc  = "".to_string();
    for char in original.chars() {
       if char as u8 <= 'Z' as u8 && char >= 'A' {
            cc.push(( 'A' as u8 + ('Z' as u8 - char as u8)) as char);
       } else if char as u8 <= 'z' as u8 && char >= 'a' {
            cc.push(( 'a' as u8 + ('z' as u8 - char as u8)) as char);
       }else{
           cc.push(char);
       }
    }
    if &cc == ciphered {
        return Ok(());
    }else{
        return Err(CipherError{ expected: format!("{}", cc)});
    }
}
