use std::fmt;
use std::collections::HashMap;
use bounded_integer::BoundedU8;
use rand::Rng;
use rand_distr::{Normal, Distribution};
use once_cell::sync::OnceCell;
use sdk::*;

pub mod sdk;

// TODO - import and refactor to use anyhow for error handling, and maybe also use thiserror to
//        enable use of #[derive(Error, Debug)] for any custom errors.
// TODO - (Don't, as deref should only be implemented for pointers)
//        define deref trait so that we don't need the .0 in cards.0.len() etc.
//        Umm... Cancel the above, for predictability reasons (per the API guidance)
//        deref should *only* by defined for smart pointers.  Instead, the desired
//        methods (len, push, pop etc...) should be defined for the newtype
//        seems non-ideal... or, just keep the destructuring/.0 bit in the usage.
//        so: in the crate (where visible via an api), implement all the functions
//        whereas internally, keep the .0 for the new types which don't appear in
//        the api.
//        Another Idea:  Create a trait for Newtype Vec which unwraps the newtype to make
//        All of the methods that work on a vector, work on the newtype.
// TODO - Need to better understand how to do math with bounded values.  At this point it
//        appears that some/all math that includes bounded values gets checking whereas
//        if you extract raw values, do math, create new bounded values it works as expected.
//        the issue might be just if you are saving to an existing bounded value - and maybe
//        this is to ensure no out-of-bounds intermediate values but, as it is, I'm clearly
//        not using it right.  and it might be best to make a "raw_value" method for all of them
//        and always do the math with a raw value and create a new bounded value with the result.
// TODO - Add tests including Ensure all works with up to six decks of cards
// TODO - Also add trait or whatever so that user can define their own custom deck
//        May have to distinguish sequence value from score (point?) value
// TODO - Rethink bounded for card values as they could be set by users...
// TODO - Figure out how to have the Cards user define the bounding for the values put in the
//        Table so that Cards can do bounds checking internally - or, get rid of value bounds
//        checking.
// TODO - Is user required to initialize value table even if they don't use it?

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
pub enum JokerId{
    A,
    B,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

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
    Card::Joker(JokerId::A),
    Card::Joker(JokerId::B),
];

impl fmt::Display for JokerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JokerId::A => write!(f, "A"),
            JokerId::B => write!(f, "B"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Card {
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

type DefCardValue = BoundedU8<1, 54>;  // default card value

static DEFAULT_VALUES: OnceCell<HashMap<Card, DefCardValue>> = OnceCell::new();

impl Card {
    pub fn default_value(&self) -> DefCardValue {
        // can panic if value table init code broken - not all cards included
        *DEFAULT_VALUES.get_or_init(|| {Cards::default_value_init()}).get(self).unwrap()
    }

    pub fn next_def_val_in_sequence(&self, jokers_per_deck: JokersPerDeck) -> DefCardValue {
        let last_val_in_new_deck = match i32::from(jokers_per_deck) {
            0 => Card::Ace(Suit::Spade).default_value(),
            1 => Card::Joker(JokerId::A).default_value(),
            2 => Card::Joker(JokerId::B).default_value(),
            _ => Card::Ace(Suit::Spade).default_value(),
        };
        // can panic if value table or value bounds code broken
        if u8::from(self.default_value())  < u8::from(last_val_in_new_deck) {
            DefCardValue::new(u8::from(self.default_value()) +1).unwrap()
        } else {
            DefCardValue::new(1).unwrap()
        }
    }

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


pub type NoiseLevel = BoundedU8<0, 10>;
pub type JokersPerDeck = BoundedU8<0, 2>;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Cards (
    pub Vec<Card>,
);

// new deck order (per above):
// hearts A, 2-K, clubs A, 2-K, Diamonds K-2, A, Spades K-2, A, Joker A, Joker B
impl Cards {
    pub fn new(count: usize, jokers_cnt: JokersPerDeck) -> Cards {
        if count == 0 {
            return Cards(vec!());
        };

        let mut deck =
            NEW_DECK_ARR[..=(51 + usize::from(jokers_cnt))].to_vec();

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

    pub fn cut(mut self, index: usize) -> TwoStacks {
        if index  >= self.0.len() {
            return TwoStacks(self.clone(), Cards(vec!()));
        }
        let bottom = Cards(self.0.split_off(index));
        TwoStacks(self.clone(), bottom)
    }

    // noise == 0 => exact cut after first half (even count) or
    // 50/50 chance of the middle card (odd count) in the first or
    // second half of the cut.
    // for noise in 1 - 10 (inclusive) cut location is a normal
    // distribution with mean at the center point and Variance approximately
    // smoothly varying from 1 to the number of cards (i.e. Standard Deviation
    // = 1 + (NoiseLevel - 1) * (Sqrt(number of cards))/9)
    // the cut point is the index of the card after which we will cut -
    // A cut point of 0 means the whole goes after the cut
    pub fn cut_with_noise(self, noise: NoiseLevel) -> TwoStacks {
        if noise == NoiseLevel::new(0).unwrap() {
            let count = self.0.len();
            return self.cut(count/2);
        } else {
            let count = self.0.len() as f64;
            let noise: i16 = noise.into();
            let noise: f64 = noise.into();
            let sd = 1.0 + (noise - 1.0) * (f64::sqrt(count) - 1.0) / 9.0;
            //  can panic if sd calc above broken which leads to a non-finite number
            let normal = Normal::new(count / 2.0, sd).unwrap();
            let cut_point = normal.sample(&mut rand::thread_rng());
            let cut_point = cut_point as isize;
            let cut_point = match cut_point {
                cp if cp < 0 => 0,
                cp if cp > count as isize => count as usize,
                cp => cp as usize,
            };
            self.cut(cut_point)
        }
    }

    pub fn shuffle(&mut self, riffle_count: usize, noise: NoiseLevel) {
        for _ in 0..riffle_count {
            // if shuffle noise is off (i.e. 0) use a "perfect" IN merge.
            // A perfect in merge of 52 cards should return deck to it's original state after
            // 52 shuffles (whereas it only takes 8 perfect OUT shuffles to do so)
            let m_type:MergeType;
            m_type = match u8::from(noise) {
                0 => MergeType::IN,
                _ => MergeType::RANDOM,
            };
            *self = self.clone().cut_with_noise(noise).merge(m_type);
        }
    }

    // Perfectly randomized card ordering
    pub fn shuffle_fy(&mut self) {  // Fisher-Yates algo from Wikipedia
        let mut rng = rand::thread_rng();
        let n = self.0.len();
        for i in 0..(n - 2) {
            self.0.swap(i, rng.gen_range(i..n));
        }
    }

    // required to init values for *all* possible cards
    fn default_value_init() -> HashMap<Card, DefCardValue> {
        let mut values = HashMap::new();
        // can panics if next line broken - illegal value for JokersPerDeck
        let new_deck = Cards::new(1,JokersPerDeck::new(2).unwrap()); // new deck w/ Joker -> 54 cards
        for (i, card) in new_deck.0.iter().enumerate() {
            // can panic if next line broken - illegal card value.
            values.insert(*card, DefCardValue::new((i + 1) as u8).unwrap());  // values not zero based
        }
        values
    }

    // Rising sequence count metric from numerous sources with modifications for jokers and multiple decks
    // e.g. "Shuffling Study.pdf" Caedmon.  Note, max length rising sequences include "sequences" of just
    // a single value (not obvious why but that's the way they are counted in the literature...) and
    // the closer the value is to the deck size/2 seems to be the actual metric (bell curve and all that)
    // //https://math.stackexchange.com/questions/4354898/how-can-you-measure-how-shuffled-a-deck-of-cards-is
    // https://drive.google.com/file/d/1EoJhtHAO5iFjikkH35KDVrmmJQpXVb5q/view?usp=sharing
    // Note that my adaptation for the inclusion of one or more jokers per deck and the use of
    // multiple decks will lead to different values for the same level of shuffling of the one-deck
    // no joker case (which can be checked by comparing riffle shuffling with Fisher-Yates).
    // Presence of one or Jokers per deck determined by modulo 52 calculation.
    pub fn shuffle_rs_metric(&self) -> usize {
        let deck_cnt = self.0.len() / 52;
        let mut jokers_per_deck = (self.0.len() % 52) / deck_cnt;
        if jokers_per_deck > 2 {
            jokers_per_deck = 0;  // if non-complete decks being used - assume no jokers
        }
        // can panic if bounds limiting code above broken
        let jokers_per_deck = JokersPerDeck::new(jokers_per_deck as u8).unwrap();
        let mut n: usize = 0;
        let mut in_sequence = vec![false; self.0.len()];
        for (i, start) in self.0[0..self.0.len()-1].iter().enumerate() {
            if !in_sequence[i] {
                in_sequence[i] = true;
                n += 1;
            }
            let mut this = start;
            for (k, candidate_card) in self.0[i + 1..].iter().enumerate() {
                if usize::from(candidate_card.default_value()) == usize::from(this.next_def_val_in_sequence(jokers_per_deck)) {
                    if !in_sequence[i + k + 1] {
                        in_sequence[i + k + 1] = true;
                        this = candidate_card;
                    }
                }
            }
        }
        n
    }

    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    pub fn move_card(&mut self, card: Card, match_index: usize, position_change: isize) -> bool {
        let Some(position_start) = self.0.iter()
            .enumerate()
            .filter_map(|(idx, r)| (*r == card).then(|| idx))
            .nth(match_index)
            else {
                return false;
            };
        let mut position_end
            = (position_start as isize + position_change).rem_euclid(self.0.len() as isize) as usize;

        let card = self.0.remove(position_start);

        // adjust for vector index offset caused by removing the card
        if position_end > position_start {
            position_end += 1;
        }

        // ensure we are still in the range vec insert requires (corner case at end of deck)
        position_end = position_end.rem_euclid(self.0.len() + 1);

        self.0.insert(position_end, card);

        true
    }

    pub fn move_card_circular(&mut self, card: Card, match_index: usize, position_change: isize)
                          -> bool {
        let Some(position_start) = self.0.iter()
            .enumerate()
            .filter_map(|(idx, r)| (*r == card).then(|| idx))
            .nth(match_index)
            else {
                return false;
            };
        let mut position_end
            = (position_start as isize + position_change).rem_euclid(self.0.len() as isize) as usize;

        // perform wrap around adjustment (i.e. as if cards are in a circle, not a stack)
        // if position change is positive and position_end is less than position start, we need to
        // add one (since there is no card to skip over between the last and first in a stack as we
        // wrap around.  Similarly if the position change is negative and position_end is greater
        // than the position start, we need to subtract one,

        if position_change > 0 && position_end < position_start {
            position_end += 1;
        } else if position_change < 0 && position_end > position_start {
            position_end -= 1;
        }

        let card = self.0.remove(position_start);

        self.0.insert(position_end, card);

        true
    }

    pub fn draw_count(&mut self, count: usize) -> Result<Cards, String> {
        if count > self.0.len() {
            return Err("Can not draw more than are available".to_string());
        }
        Ok(Cards(self.0.drain(0..count).collect()))
    }

    pub fn draw_till(&mut self, card: Card) -> Result<Cards, String> {
        let count = self.0.iter().position(|r| *r == card).ok_or("Card not found")?;
        // can panic if index limiting code above broken
        Ok(self.draw_count(count).unwrap())
    }

    pub fn append(&mut self, mut cards: Cards) {
        self.0.append(&mut cards.0);
    }

    pub fn look_at(&self, index: usize) -> Result<&Card, String> {
        if index >= self.0.len() {
            return Err("Index beyond end of Cards".to_string());
        }
        Ok(&self.0[index])
    }

    pub fn by_def_raw_values(&self) -> Vec<u8> {
        let values: Vec<u8> = self.0.iter().map(|c| u8::from((*c).default_value())).collect();
        values
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

pub struct TwoStacks (pub Cards, pub Cards);

#[derive(PartialEq)]
pub enum MergeType {
    IN,
    OUT,
    RANDOM,
}
impl TwoStacks {
    pub fn merge(self, m_type: MergeType) -> Cards {
        let TwoStacks(mut top, mut bottom) = self;
        let mut cards = Cards::default();
        let mut rng = rand::thread_rng();
        for i in 0..(&top.0.len() + &bottom.0.len()) {
            let first_try: &mut Vec<Card>;
            let then_try: &mut Vec<Card>;
            // Reminder - we are popping from the bottom of the stacks and later reversing
            // So an IN merge will result in the last card of the top stack on the bottom
            if m_type == MergeType::IN && (i % 2) == 0
                || m_type == MergeType::OUT && (i % 2) == 1
                || m_type == MergeType::RANDOM && rng.gen() {
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
        // act of push cards popped from effectively reverses the order of the resulting deck
        // which has to be corrected to match actions of a traditional shuffle
        cards.reverse();
        cards
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_cards() {
        let deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
        assert_eq!(deck.0.len(), 54, "new deck with jokers wrong length");
        for (i, card) in deck.0.iter().enumerate() {
            assert_eq!(*card, NEW_DECK_ARR[i], "new deck cards don't match ref array");
        }
        println!("New deck: {deck}, shuffle rising sequence metric: {}", deck.shuffle_rs_metric());
        println!("by default values:\n {:?}", deck.by_def_raw_values());
        println!();
    }

    #[test]
    fn test_raw_values() {
        let deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
        for (i, value) in deck.by_def_raw_values().iter().enumerate() {
            assert_eq!(usize::from(*value), i + 1, "broken raw value function");
        }
    }
}
