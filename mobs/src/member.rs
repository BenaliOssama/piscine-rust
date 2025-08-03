#[derive(Copy, Clone, Debug, PartialEq)]


pub enum Role {
    Underboss,

    Caporegime,

    Soldier,

    Associate,

}


#[derive(  Clone, Copy, Debug, PartialEq)]
pub struct Member {
    pub role: Role,
    pub age: u32,
}


impl Member {
    pub fn get_promotion(&mut self) {
        self.role = match self.role {
            Role::Soldier => Role::Caporegime,
            Role::Caporegime => Role::Underboss,
            Role::Associate => Role::Soldier,

            _ => unreachable!(),
        }
    }
}
