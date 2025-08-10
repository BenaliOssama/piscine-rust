use std::fmt::Formatter;


#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> { 
        let mut antigine = Antigen::A;
        match s {
            "A"=> antigine = Antigen::A,
            "B"=> antigine = Antigen::B,
            "AB"=> antigine = Antigen::AB,
            "O"=> antigine = Antigen::O,
            other => {
                return Err(format!("`{}` is not a valid antigen", other));
            },

        }
        return Ok(antigine);// antigine};
    }
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> { 
        if s.trim() == "+" {
            return Ok(RhFactor::Positive);
        }else if s.trim() == "-"{
            return Ok(RhFactor::Negative);
        }else{
            return Err(String::from("no rh type"));
        }
    }

}

impl Ord for BloodType {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering { 
        todo!() 
    }
}

impl FromStr for BloodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> { 
        let mut antigine = Antigen::A; 
        let mut rh_factor = RhFactor::Positive; 
        
        let mut blood : String = String::new();

        if s.contains("+") {
            blood = s.to_string().replace("+", "");
            rh_factor = RhFactor::Positive;
        } else if s.contains("-") {
            blood = s.to_string().replace("-", "");
            rh_factor = RhFactor::Negative;
        }else{
            return Err(String::from("nothing"));
        }

        match blood.as_str() {
            "A"=> antigine = Antigen::A,
            "B"=> antigine = Antigen::B,
            "AB"=> antigine = Antigen::AB,
            "O"=> antigine = Antigen::O,
            _ => (),
        }
        return  Ok(BloodType{antigen: antigine, rh_factor: rh_factor});
    
    }
}
//
use std::fmt::{self, Debug};
//
impl Debug for BloodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        let rh = if self.rh_factor == RhFactor::Positive{
            "+"
        }else{
            "-"
        };

        write!(f, "{:?}{}", self.antigen, rh)?;
        Ok(())
    }
}
//
impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        // Check antigen compatibility
        let antigen_ok = match self.antigen {
            Antigen::AB => true, // AB gets from everyone
            Antigen::A  => matches!(other.antigen, Antigen::A | Antigen::O),
            Antigen::B  => matches!(other.antigen, Antigen::B | Antigen::O),
            Antigen::O  => matches!(other.antigen, Antigen::O),
        };

        // Check Rh compatibility: + can get from + and -, - can only get from -
        let rh_ok = match self.rh_factor {
            RhFactor::Positive => true,
            RhFactor::Negative => matches!(other.rh_factor, RhFactor::Negative),
        };

        antigen_ok && rh_ok
    }

pub fn donors(&self) -> Vec<Self> {
    // Who can donate to self
    let all = Self::all_blood_types();
    all.into_iter().filter(|bt| self.can_receive_from(bt)).collect()
}

pub fn recipients(&self) -> Vec<Self> {
    // Who I can donate to
    let all = Self::all_blood_types();
    all.into_iter().filter(|bt| bt.can_receive_from(self)).collect()
}
    fn all_blood_types() -> Vec<Self> {
        let antigens = vec![Antigen::O, Antigen::A, Antigen::B, Antigen::AB];
        let rhs = vec![RhFactor::Negative, RhFactor::Positive];
        let mut types = Vec::new();
        for a in &antigens {
            for r in &rhs {
                types.push(Self { antigen: a.clone(), rh_factor: r.clone() });
            }
        }
        types
    }
}

