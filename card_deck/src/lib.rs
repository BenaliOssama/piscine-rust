pub enum Suit {
}

pub enum Rank {
}

impl Suit {
    pub fn random() -> Suit {
    todo!()
    }

    pub fn translate(value: u8) -> Suit {
    todo!()
    }
}

impl Rank {
    pub fn random() -> Rank {
    todo!()
    }

    pub fn translate(value: u8) -> Rank {
    todo!()
    }
}
#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    todo!()
}
