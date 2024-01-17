use std::fmt;
use std::collections::HashMap;
use bounded_integer::BoundedU8;
use rand_distr::{Normal, Distribution};
use rand::Rng;

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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Club => write!(f, "C"),
            Suit::Diamond => write!(f, "D"),
            Suit::Heart => write!(f, "H"),
            Suit::Spade => write!(f,  "S"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum JokerId{
    A,
    B,
}

impl fmt::Display for JokerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JokerId::A => write!(f, "A"),
            JokerId::B => write!(f, "B"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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
            Card::Ace(ref s) => write!(f, "A{}", s), 
            Card::Two(ref s) => write!(f, "2{}", s),
            Card::Three(ref s) => write!(f, "3{}", s),
            Card::Four(ref s) => write!(f, "4{}", s),
            Card::Five(ref s) => write!(f, "5{}", s),
            Card::Six(ref s) => write!(f, "6{}", s),
            Card::Seven(ref s) => write!(f, "7{}", s),
            Card::Eight(ref s) => write!(f, "8{}", s),
            Card::Nine(ref s) => write!(f, "9{}", s),
            Card::Ten(ref s) => write!(f, "T{}", s),
            Card::Jack(ref s) => write!(f, "J{}", s),
            Card::Queen(ref s) => write!(f, "Q{}", s),
            Card::King(ref s) => write!(f, "K{}", s),
            Card::Joker(ref id) => write!(f, "F{}", id),
        }
    }
}

type NoiseLevel = BoundedU8<0, 10>;

#[derive(PartialEq, Clone)]
enum DeckStyle {
    NoJokers,
    Jokers
}

#[derive(PartialEq, Clone, Default)]
struct Cards (
    Vec<Card>,
);

// new deck order from *back* side (i.e. as usually dealt)
// hearts A, 2-K, clubs A, 2-K, Diamonds K-2, A, Spades K-2, A, Joker B, Joker A 
impl Cards {
    fn new(style: DeckStyle, count: usize) -> Cards {
        let mut deck = if style == DeckStyle::NoJokers {
            NEW_DECK_ARR[..=51].to_vec()
        } else {
            NEW_DECK_ARR[..].to_vec()
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

    // noise == 0 => exact cut after first half(even count) or
    // 50/50 chance of the middle card (out count) in the first or
    // second half of the cut.
    // for noise in 1 - 10 (inclusive) cut location is a normal 
    // distribution with mean at the center point and Variance approximately
    // smoothly varying from 1 to the number of cards (i.e. Standard Deviation
    // = 1 + (NoiseLevel - 1) * (Sqrt(number of cards))/9)
    // the cutpoint is the index of the card after which we will cut - 
    // A cutpoint of 0 means the whole goes after the cut
    fn cut(mut self, noise: NoiseLevel) -> TwoStacks {
        let count: f64 = self.0.len() as f64;
        let noise: i16 = noise.into();
        let noise: f64 = noise.into();
        let sd = 1.0 + (noise - 1.0) * (f64::sqrt(count) - 1.0)/9.0;
        let normal = Normal::new(count/2.0, sd).unwrap();
        let cutpoint = normal.sample(&mut rand::thread_rng());
        let cutpoint = cutpoint as isize;
        let cutpoint = match cutpoint {
            cp if cp < 0 => 0,
            cp if cp > count as isize => count as usize,
            cp => cp as usize,
        };
        let bottom = Cards(self.0.split_off(cutpoint));
        TwoStacks(self.clone(), bottom)
    }


    fn shuffle(mut self, riffle_count: usize, noise: NoiseLevel) -> Cards {
        for _ in 0..riffle_count {
            self = self.cut(noise).merge();
        }
        self
    }

    fn reverse(mut self) -> Cards {
        self.0.reverse();
        self
    }

    fn move_card(mut self, card: Card, position_change: isize) -> Result<Cards, String> {
        let position_start = self.0.iter().position(|r| *r == card).ok_or("Card not found")?;
        let position_end = (position_start as isize + position_change).rem_euclid(self.0.len() as isize) as usize;

        let card = self.0.remove(position_start);
        self.0.insert(position_end, card);

        Ok(self)
    }

    fn draw_count(&mut self, count: usize) -> Result<Cards, String> {
        if count > self.0.len() {
            return Err("Can not draw more than are available".to_string());
        }
        Ok(Cards(self.0.drain(0..count).collect()))
    }

    fn draw_till(&mut self, card: Card) -> Result<Cards, String> {
        let count = self.0.iter().position(|r| *r == card).ok_or("Card not found")?;
        Ok(self.draw_count(count).unwrap())
    }

    fn append(&mut self, mut cards: Cards) {
        self.0.append(&mut cards.0);
    }

    fn look_at(&self, index: usize) -> Result<&Card, String> {
        if index >= self.0.len() {
            return Err("Index beyond end of Cards".to_string());
        }
        Ok(&self.0[index])
    }
}


impl fmt::Display for Cards {
    fn  fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in self.0.iter() {
            write!(f, "{}, ", card)?;
        }
        Ok(())
    }  
}

struct TwoStacks (Cards, Cards);

impl TwoStacks {
    fn merge(self) -> Cards {
        let TwoStacks(mut top, mut bottom) = self;
        let mut rng = rand::thread_rng();
        let mut cards = Cards::default();
        for _ in 0..(&top.0.len() + &bottom.0.len()) {
            if rng.gen() {
                match top.0.pop() {
                    Some(card) => cards.0.push(card),
                    None => {
                        match bottom.0.pop() {
                            Some(card) => cards.0.push(card),
                            None => panic!("Must have loop counter wrong!"),
                        }
                    }
                }
            }  else {
                match bottom.0.pop() {
                    Some(card) => cards.0.push(card),
                    None => {
                        match top.0.pop() {
                            Some(card) => cards.0.push(card),
                            None => panic!("Must have loop counter wrong!"),
                        }
                    }
                }
            }

        }
        cards
    }
}

fn main() {
    println!("Hello World!");
    let deck = Cards::new(DeckStyle::Jokers, 1);
    println!("New deck (face down): {deck}");
    println!("");
    let deck = deck.reverse();
    println!("Reversed deck (or, if you like, orginal deck now face up): {deck}");
    println!("");
    let TwoStacks(top, bottom) = deck.cut(NoiseLevel::new(10).unwrap());
    println!("After cut top: {top}");
    println!("");
    println!("After cut bottom: {bottom}");
    println!("");
    println!("Cut point: {}", top.0.len());
    println!("");
    let deck = TwoStacks(top, bottom).merge();
    println!("after first riffle:");
    println!("{}", deck);
    println!("");
    let deck = deck.shuffle(10, NoiseLevel::new(10).unwrap());
    println!("after 10 more riffles:");
    println!("{}", deck);
    println!("");
    let deck = deck.move_card(Card::Joker(JokerId::A), 1).unwrap();
    println!("after moving Joker A (aka \"FA\") down one");
    println!("{}", deck);
    println!("");
    let mut deck = deck.move_card(Card::Joker(JokerId::B), 2).unwrap();
    println!("after moving Joker B (aka \"FB\") down one");
    println!("{}", deck);
    println!("");

    let mut above_fa = deck.draw_till(Card::Joker(JokerId::A)).unwrap();
    if let Ok(above_both) = above_fa.draw_till(Card::Joker(JokerId::B)) {
        // Order is: above_both, FB above_fa, FA deck
        let fb = above_fa.draw_count(1).unwrap();
        let fa = deck.draw_count(1).unwrap();
        // swap top and bottom leaving fools in same relative positions
        deck.append(fb);
        deck.append(above_fa);
        deck.append(fa);
        deck.append(above_both);
    } else if let Ok(mut above_fb) = deck.draw_till(Card::Joker(JokerId::B)) {
        // Order is: above_fa, FA above_fb, FB deck
        let fa = above_fb.draw_count(1).unwrap();
        let fb = deck.draw_count(1).unwrap();
        // swap top and bottom leaving fools in same relative positions
        deck.append(fa);
        deck.append(above_fb);
        deck.append(fb);
        deck.append(above_fa);
    }   
    println!("after swapping ends around the jokers (aka \"FA\", \"FB\")");
    println!("{}", deck);
    println!("");

    println!("The bottom card is: {}", deck.look_at(deck.0.len()-1).unwrap());
    // Need to make a length method so as to not expose Cards internals

    // Establish value table for the solitaire cypher
    let mut values = HashMap::new();
    let new_deck = Cards::new(DeckStyle::Jokers, 1);
    // Also, find the fix so that the 0 isn't required here, i.e. implement iterater trade for Cards
    for (i, card) in new_deck.0.iter().enumerate() {
        values.insert(*card, i + 1);  // values not zero based
    }

    // fixup the last jokers value
    values.entry(Card::Joker(JokerId::B)).and_modify(|value| *value = 53);

    // println!("");
    // println!("The value of FB is {}", values.get(&Card::Joker(JokerId::B)).unwrap());

    println!("The value of the bottom card is: {}", values.get(&deck.look_at(deck.0.len()-1).unwrap()).unwrap());
}