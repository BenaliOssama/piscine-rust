use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Player {
	pub name: String,
	pub strength: f64,
	pub score: i32,
	pub money: i32,
	pub weapons: Vec<String>,
}


impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "{}",self.name)?;
        writeln!(f, "Strength: {}, Score: {}, Money: {}",self.strength,self.score,self.money)?;
        write!(f, "Weapons: {:?}",self.weapons)?;
        Ok(())
     }
}

pub struct Fruit {
	pub weight_in_kg: f64,
}

pub struct Meat {
	pub weight_in_kg: f64,
	pub fat_content: f64,
}

impl Player {
	pub fn eat<T: Food>(&mut self, food: T) {
		self.strength += food.gives();
	}
}

pub trait Food {
	fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        return self.weight_in_kg * 4.0;
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
        let fat  = (1.0 - self.fat_content) * 0.9;
        let pro = (self.weight_in_kg - fat) * 4.0 ;

        return fat + pro ;
    }
}
