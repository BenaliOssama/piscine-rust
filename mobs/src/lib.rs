
use std::collections::{HashMap, HashSet};


pub use member::*;
pub use boss::*;

pub mod boss;
pub mod member;



#[derive(Debug ,PartialEq,  Clone)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, (name, age): (&str, u32)) {
        self.members.insert(
            name.to_owned(),
            Member {
                role: Role::Associate,
                age,
            },
        );
    }

    fn give_cities(&mut self, to: &mut Mob) {
        to.cities.extend(self.cities.drain())
    }

    fn calculate_power(&self) -> usize {
        self.members
            .values()
            .map(|m| match m.role {
                Role::Associate => 1,
                Role::Soldier => 2,
                Role::Caporegime => 3,
                Role::Underboss => 4,
            })
            .sum()
    }


    pub fn attack(&mut self, target: &mut Mob) {
        let (winner, loser) = if self.calculate_power() < target.calculate_power() {
            (target, self)
        } else {
            (self, target)
        };

        let smallest = loser.members.values().map(|m| m.age).min().unwrap();

        loser.members.retain(|n, m| m.age > smallest);

        if loser.members.is_empty() {
            loser.give_cities(winner);
            winner.wealth += loser.wealth;
            loser.wealth = 0;
        }
    }

    pub fn steal(&mut self, target: &mut Mob, value: u64) {
        let x = value.min(target.wealth);
        self.wealth += x;
        target.wealth -= x;
    }

    pub fn conquer_city(&mut self, mobs: &[&Mob], city: String) {
        if !mobs.iter().flat_map(|m| &m.cities).any(|c| *c == city){
            self.cities.insert(city);
        }
    }
}
