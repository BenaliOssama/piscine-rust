use std::fmt::{self, Debug, Formatter};
use std::cmp::Ordering;
use std::str::FromStr;

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

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            other => Err(format!("`{}` is not a valid antigen", other)),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            other => Err(format!("`{}` is not a valid Rh Factor", other)),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.antigen.cmp(&other.antigen)
            .then(self.rh_factor.cmp(&other.rh_factor))
    }
}

impl FromStr for BloodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 || s.len() > 3 {
            return Err(format!("Invalid length for blood type: `{}`", s));
        }

        let rh_factor = if s.contains('+') {
            RhFactor::Positive
        } else if s.contains('-') {
            RhFactor::Negative
        } else {
            return Err(format!("Missing Rh factor in `{}`", s));
        };

        let blood_str = s.trim_end_matches(&['+', '-'][..]);
        let antigen = Antigen::from_str(blood_str)?;

        Ok(BloodType { antigen, rh_factor })
    }
}

impl Debug for BloodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let rh = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{:?}{}", self.antigen, rh)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let antigen_ok = match self.antigen {
            Antigen::AB => true,
            Antigen::A => matches!(other.antigen, Antigen::A | Antigen::O),
            Antigen::B => matches!(other.antigen, Antigen::B | Antigen::O),
            Antigen::O => matches!(other.antigen, Antigen::O),
        };

        let rh_ok = match self.rh_factor {
            RhFactor::Positive => true,
            RhFactor::Negative => matches!(other.rh_factor, RhFactor::Negative),
        };

        antigen_ok && rh_ok
    }

    pub fn donors(&self) -> Vec<Self> {
        let all = Self::all_blood_types();
        all.into_iter().filter(|bt| self.can_receive_from(bt)).collect()
    }

    pub fn recipients(&self) -> Vec<Self> {
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

