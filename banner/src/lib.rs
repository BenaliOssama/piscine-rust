use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    // expected public fields
    short_hand: String,
    long_hand: String,
    desc: String,
}

impl Flag {
    pub fn opt_flag(name: & str, d: & str) -> Self {
        return Flag{
            short_hand: name.chars().next().unwrap().to_string(),
            long_hand: name.to_string(),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert( flag.long_hand, func);
        self.flags.insert(flag.short_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let mut arg : String =  String::from(input);
        let arg = arg.replace("-", "");
        let callback = self.flags.get(&arg).unwrap();
        match callback(argv[0], argv[1]) {
            Ok(res) => Ok(res),
            Err(res) => Err(String::from("invalid float literal")),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let na = a.parse::<f64>()?;
    let nb = b.parse::<f64>()?;
    let res = (na/nb).to_string(); 
    Ok(res)
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let na = a.parse::<f64>()?;
    let nb = b.parse::<f64>()?;
    let res = (na % nb).to_string(); 
    Ok(res)
}
