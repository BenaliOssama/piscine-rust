use rand::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let mut Deck : Vec<Suit> = vec![Suit::Heart, Suit::Diamond, Suit::Spade, Suit::Club];
        Deck.shuffle(&mut rng);
        Deck[0]
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond, 
            3 => Suit::Spade, 
            4 => Suit::Club, 
            _ => panic!("over the limit")
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let mut rank : Vec<Rank> = vec![Rank::Ace, Rank::King, Rank::Queen, Rank::Jack];
        for i in 2..=10 {
            rank.push(Rank::Number(i as u8));
        }
        rank.shuffle(&mut rng);
        rank[0]
    }

    pub fn translate(value: u8) -> Rank {
        match value {
                2..=10 => Rank::Number(value),
                1 =>  Rank::Ace , 
                11 => Rank::Jack, 
                12 => Rank::Queen, 
                13 => Rank::King,
                _ => panic!("rank over the limit")
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    //card == &Card{suit:Suit::Spade, rank:Rank::Ace}
    match card.suit {
        Suit::Spade =>  
        {
            match card.rank {
                Rank::Ace => true,
                _ => false ,
            }
        },
        _ => false,
    }
}
