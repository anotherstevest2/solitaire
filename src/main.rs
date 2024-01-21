use std::fmt;
use std::collections::HashMap;
use bounded_integer::BoundedU8;
use rand_distr::{Normal, Distribution};
use rand::Rng;
use once_cell::sync::OnceCell;

// TODO - import and refactor to use anyhow for error handling;
// TODO - define deref trait so that we don't need the .0 in cards.0.len() etc.

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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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

    fn cut(mut self, index: usize) -> TwoStacks {
        if index  >= self.0.len() {
            panic!("indexed past the end of Cards!");
        }
        let bottom = Cards(self.0.split_off(index));
        TwoStacks(self.clone(), bottom)
    }

    // noise == 0 => exact cut after first half(even count) or
    // 50/50 chance of the middle card (out count) in the first or
    // second half of the cut.
    // for noise in 1 - 10 (inclusive) cut location is a normal 
    // distribution with mean at the center point and Variance approximately
    // smoothly varying from 1 to the number of cards (i.e. Standard Deviation
    // = 1 + (NoiseLevel - 1) * (Sqrt(number of cards))/9)
    // the cut point is the index of the card after which we will cut -
    // A cut point of 0 means the whole goes after the cut
    fn cut_with_noise(self, noise: NoiseLevel) -> TwoStacks {
        let count: f64 = self.0.len() as f64;
        let noise: i16 = noise.into();
        let noise: f64 = noise.into();
        let sd = 1.0 + (noise - 1.0) * (f64::sqrt(count) - 1.0)/9.0;
        let normal = Normal::new(count/2.0, sd).unwrap();
        let cut_point = normal.sample(&mut rand::thread_rng());
        let cut_point = cut_point as isize;
        let cut_point = match cut_point {
            cp if cp < 0 => 0,
            cp if cp > count as isize => count as usize,
            cp => cp as usize,
        };
        self.cut(cut_point)
    }

    fn shuffle(mut self, riffle_count: usize, noise: NoiseLevel) -> Cards {
        for _ in 0..riffle_count {
            self = self.cut_with_noise(noise).merge();
        }
        self
    }

    fn reverse(mut self) -> Cards {
        self.0.reverse();
        self
    }

    fn move_card(&mut self, card: Card, position_change: isize) -> Result<Cards, String> {
        let position_start = self.0.iter().position(|r| *r == card).ok_or("Card not found")?;
        let position_end = (position_start as isize + position_change).rem_euclid(self.0.len() as isize) as usize;

        let card = self.0.remove(position_start);
        self.0.insert(position_end, card);

        Ok(self.clone())
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

    fn append(&mut self, mut cards: Cards) -> &mut Self {
        self.0.append(&mut cards.0);
        self
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
            let first_try: &mut Vec<Card>;
            let then_try: &mut Vec<Card>;
            if rng.gen() {
                first_try = &mut top.0;
                then_try = &mut bottom.0;
            } else {
                first_try = &mut bottom.0;
                then_try = &mut top.0;
            }

            if let Some(card) = first_try.pop() {cards.0.push(card)}
            else if let Some(card) = then_try.pop() {cards.0.push(card)}
            else {panic!("Must have loop counter wrong!");}
        }
        cards
    }
}

trait Value {
    fn value(&self) -> CardValue;
}

// ----------------- TODO - Cards crate above, Solitaire Cypher Crate Below ----------------------
// When refactoring - consider doing newtype pattern on Bounded variables so that from can be
// defined.

type UpperLetter = BoundedU8<65, 90>;

fn upper_into_let_value(ul: UpperLetter) -> LetterValue {
    LetterValue::new((ul - 64).into()).unwrap()
}

type LetterValue = BoundedU8<1, 26>;


type CardValue = BoundedU8<1, 53>;

fn card_val_into_let_val(cv: CardValue) -> LetterValue {
    LetterValue::new(((cv - 1) % 26 + 1).into()).unwrap()
}
struct PlainText (Vec<UpperLetter>);

impl PlainText {
    fn new() -> PlainText {
        PlainText(Vec::new())
    }
}
struct CypherText (Vec<UpperLetter>);
impl CypherText {
    fn new() -> CypherText {
        CypherText(Vec::new())
    }
}

struct KeyStream (Vec<LetterValue>);

impl KeyStream {
    fn new() -> KeyStream {
        KeyStream(Vec::new())
    }
}

static VALUES: OnceCell<HashMap<Card, CardValue>> = OnceCell::new();

impl Value for Card {
    fn value(&self) -> CardValue {
        *VALUES.get_or_init(|| {value_init()}).get(self).unwrap()
    }
}

fn value_init() -> HashMap<Card, CardValue> {
    let mut values = HashMap::new();
    let new_deck = Cards::new(DeckStyle::Jokers, 1); // new deck w/ Joker -> 54 cards
    for (i, card) in new_deck.0.iter().enumerate() {
        if i < 53 {  // have to skip last card as it will generate illegal value
            values.insert(*card, CardValue::new((i + 1) as u8).unwrap());  // values not zero based
        }
    }
    // add the last joker with same (53) value as the other one
    values.insert(Card::Joker(JokerId::B), CardValue::new(53u8).unwrap());
    values
}

fn main() {
    VALUES.get_or_init(|| {value_init()});

    // key_deck is interpreted as *face-up*
    fn get_key_stream(key_deck: Cards, key_length: usize) -> KeyStream {
        let mut key_deck = key_deck;
        let mut key_stream = KeyStream::new();
        while key_stream.0.len() < key_length {
            // A Joker move
            let mut key_deck = key_deck.move_card(Card::Joker(JokerId::A), 1).unwrap();
            // B Joker move
            key_deck = key_deck.move_card(Card::Joker(JokerId::B), 2).unwrap();

            // Triple cut at Jokers (aka fools. fa, fb being fool A and fool B respectively)
            // and swap top with bottom leaving Jokers in place
            let mut above_fa = key_deck.draw_till(Card::Joker(JokerId::A)).unwrap();
            if let Ok(above_both) = above_fa.draw_till(Card::Joker(JokerId::B)) {
                // Order is: above_both, FB above_fa, FA key_deck
                let fb = above_fa.draw_count(1).unwrap();
                let fa = key_deck.draw_count(1).unwrap();
                // swap top and bottom leaving fools in same relative positions
                key_deck.append(fb);
                key_deck.append(above_fa);
                key_deck.append(fa);
                key_deck.append(above_both);
            } else if let Ok(mut above_fb) = key_deck.draw_till(Card::Joker(JokerId::B)) {
                // Order is: above_fa, FA above_fb, FB key_deck
                let fa = above_fb.draw_count(1).unwrap();
                let fb = key_deck.draw_count(1).unwrap();
                // swap top and bottom leaving fools in same relative positions
                key_deck.append(fa);
                key_deck.append(above_fb);
                key_deck.append(fb);
                key_deck.append(above_fa);
            }

            // Count cut based on the value of the bottom card leaving the bottom card at the bottom
            let bottom_card_value = &key_deck.
                look_at(key_deck.0.len() - 1)
                .unwrap()
                .value();
            let TwoStacks(top, mut bottom) = key_deck.cut(bottom_card_value.checked_add(1).unwrap().into());
            let bottom_card = bottom.0
                .pop()
                .unwrap();
            let key_deck = bottom
                .append(top)
                .append(Cards(vec!(bottom_card)));

            // Find output card, or Joker
            let top_card_value = &key_deck
                .look_at(1)
                .unwrap()
                .value();
            let output_card_candidate = &key_deck
                .look_at(top_card_value.checked_add(1).unwrap().into()).unwrap();
            if  **output_card_candidate != Card::Joker(JokerId::A)
            && **output_card_candidate != Card::Joker(JokerId::B) {
                key_stream.0
                .push(card_val_into_let_val((*output_card_candidate).value()));
            }
        }
        key_stream
    }

    let deck = Cards::new(DeckStyle::Jokers, 1);
    println!("New deck (face down): {deck}");
    println!();
    let deck = deck.reverse();
    println!("Reversed deck (or, for our purposes, original deck now face up): {deck}");
    println!();
    let TwoStacks(top, bottom) = deck.cut_with_noise(NoiseLevel::new(10).unwrap());
    println!("After cut top: {top}");
    println!();
    println!("After cut bottom: {bottom}");
    println!();
    println!("Cut point: {}", top.0.len());
    println!();
    let deck = TwoStacks(top, bottom).merge();
    println!("after first riffle:");
    println!("{}", deck);
    println!();
    let mut deck = deck.shuffle(10, NoiseLevel::new(10).unwrap());
    println!("after 10 more riffles:");
    println!("{}", deck);
    println!();
    deck = deck.move_card(Card::Joker(JokerId::A), 1).unwrap();
    println!("after moving Joker A (aka \"FA\") down one");
    println!("{}", deck);
    println!();
    deck = deck.move_card(Card::Joker(JokerId::B), 2).unwrap();
    println!("after moving Joker B (aka \"FB\") down one");
    println!("{}", deck);
    println!();

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
    println!();

    println!("The bottom card is: {}", deck.look_at(deck.0.len()-1).unwrap());
    // Need to make a length method so as to not expose Cards internals

    // println!("");
    // println!("The value of FB is {}", values.get(&Card::Joker(JokerId::B)).unwrap());

    // let bottom_card_value = values.get(&deck.look_at(deck.0.len()-1).unwrap()).unwrap();
    let bottom_card_value = &deck.look_at(deck.0.len()-1).unwrap().value();
    println!("The value of the bottom card is: {}", bottom_card_value);

    let TwoStacks(top, mut bottom) = deck.cut(bottom_card_value.checked_add(1).unwrap().into());

    let bottom_card = bottom.0.pop().unwrap();
    let deck = bottom.append(top).append(Cards(vec!(bottom_card)));

    // let top_card_value = values.get(&deck.look_at(1).unwrap()).unwrap();
    let top_card_value = &deck.look_at(1).unwrap().value();
    let output_card_candidate = &deck.look_at(top_card_value.checked_add(1).unwrap().into()).unwrap();
    let output_card: Option<&Card>;
    if  **output_card_candidate != Card::Joker(JokerId::A) && **output_card_candidate != Card::Joker(JokerId::B) {
        output_card = Some(*output_card_candidate);
    } else { output_card = None; }

    println!("output_card: {:?}", output_card);

}
