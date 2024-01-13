use std::fmt;

const NEW_DECK_ARR: [Card; 54] = [
    Card::Ace(Suit::Heart),
    Card::Two(Suit::Heart),
    Card::Three(Suit::Heart),
    Card::Four(Suit::Heart),
    Card::Five(Suit::Heart),
    Card::Six(Suit::Heart),
    Card::Seven(Suit::Heart),
    Card::Eight(Suit::Heart),
    Card::Nine(Suit::Heart),
    Card::Ten(Suit::Heart),
    Card::Jack(Suit::Heart),
    Card::Queen(Suit::Heart),
    Card::King(Suit::Heart),
    Card::Ace(Suit::Club),
    Card::Two(Suit::Club),
    Card::Three(Suit::Club),
    Card::Four(Suit::Club),
    Card::Five(Suit::Club),
    Card::Six(Suit::Club),
    Card::Seven(Suit::Club),
    Card::Eight(Suit::Club),
    Card::Nine(Suit::Club),
    Card::Ten(Suit::Club),
    Card::Jack(Suit::Club),
    Card::Queen(Suit::Club),
    Card::King(Suit::Club),
    Card::King(Suit::Diamond),
    Card::Queen(Suit::Diamond),
    Card::Jack(Suit::Diamond),
    Card::Ten(Suit::Diamond),
    Card::Nine(Suit::Diamond),
    Card::Eight(Suit::Diamond),
    Card::Seven(Suit::Diamond),
    Card::Six(Suit::Diamond),
    Card::Five(Suit::Diamond),
    Card::Four(Suit::Diamond),
    Card::Three(Suit::Diamond),
    Card::Two(Suit::Diamond),
    Card::Ace(Suit::Diamond),
    Card::King(Suit::Spade),
    Card::Queen(Suit::Spade),
    Card::Jack(Suit::Spade),
    Card::Ten(Suit::Spade),
    Card::Nine(Suit::Spade),
    Card::Eight(Suit::Spade),
    Card::Seven(Suit::Spade),
    Card::Six(Suit::Spade),
    Card::Five(Suit::Spade),
    Card::Four(Suit::Spade),
    Card::Three(Suit::Spade),
    Card::Two(Suit::Spade),
    Card::Ace(Suit::Spade),
    Card::Joker(JokerId::B),
    Card::Joker(JokerId::A),
];

#[derive(PartialEq, Clone)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Club => write(f, "C"),
            Suit::Diamond => write(f, "D"),
            Suit::Heart => write(f, "H"),
            Suit::Spade => write(f,  "S"),
        }
}

#[derive(PartialEq, Clone)]
enum JokerId{
    A,
    B,
}

impl fmt::Display for JokerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JokerId::A => write(f, "A"),
            JokerId::B => write(f, "B"),
        }
    }
}

#[derive(PartialEq, Clone)]
enum Card {
    Ace(Suit),
    Two(Suit),
    Three(Suit),
    Four(Suit),
    Five(Suit),
    Six(Suit),
    Seven(Suit),
    Eight(Suit),
    Nine(Suit),
    Ten(Suit),
    Jack(Suit),
    Queen(Suit),
    King(Suit),
    Joker(JokerId),
}

impl fmt::Display for Card {
    fn  fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Card::Ace(ref s) => write(f, "A{}", s), 
            Card::Two(ref s) => write(f, "2{}", s),
            Card::Three(ref s) => write(f, "3{}", s),
            Card::Four(ref s) => write(f, "4{}", s),
            Card::Five(ref s) => write(f, "5{}", s),
            Card::Six(ref s) => write(f, "6{}", s),
            Card::Seven(ref s) => write(f, "7{}", s),
            Card::Eight(ref s) => write(f, "8{}", s),
            Card::Nine(ref s) => write(f, "9{}", s),
            Card::Ten(ref s) => write(f, "T{}", s),
            Card::Jack(ref s) => write(f, "J{}", s),
            Card::Queen(ref s) => write(f, "Q{}", s),
            Card::King(ref s) => write(f, "K{}", s),
            Card::Joker(ref id) => write(f, "J{}", id),
        }
}

#[derive(PartialEq, Clone)]
enum DeckStyle {
    NoJokers,
    Jokers
}

struct Cards (
    Vec<Card>,
);

// new deck order from *back* side (i.e. as usually dealt)
// hearts A, 2-K, clubs A, 2-K, Diamonds K-2, A, Spades K-2, A, Joker B, Joker A 
impl Cards {
    fn new(style: DeckStyle, count: usize) -> Cards {
        let mut deck = if style == DeckStyle::NoJokers {
            NEW_DECK_ARR[..].to_vec()
        } else {
            NEW_DECK_ARR[..=51].to_vec()
        };

        
        assert!(count > 0, "At least one deck is required");

        if count > 1 {
            let mut additional: Vec<Card> = Vec::new();
            for _ in 2..count {
                for card in deck.iter() {
                    additional.push(card.clone());
                }
            }
            deck.append(&mut additional);
        }
        
        Cards(deck)
    }

}

imp fmt::Display for Cards {
    fn  fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in Cards.iter() {
            write(f, "{}, ", card)
        }  
}



fn main() {
    println!("Hello World!");    
}