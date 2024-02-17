//! # Card Play
//!
//! A set of types, methods and functions for manipulating playing cards (common french-suited with or
//! without jokers). Support for cutting, merging, both human style and fully random ordering shuffles,
//! measuring shuffle quality (rising sequence based), drawing cards, moving cards in a deck
//! etc.

use bounded_integer::BoundedU8;
use once_cell::sync::OnceCell;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// TODO - Add tests including Ensure all works with up to six decks of cards
// TODO - Maybe... Also add trait or whatever so that user can define their own custom deck
//        May have to distinguish sequence value from score (point?) value
// TODO - Figure out how to have the Cards user define the bounding for the values put in the
//        Table so that Cards can do bounds checking internally - or, get rid of value bounds
//        checking.
// TODO - Is user required to initialize value table even if they don't use it?
// TODO - Reorganize and add tests to have: unit tests, integration tests and documentation tests.
// TODO - Decide if Release mode should have panic = 'abort' instead of unwinding (smaller executable)
// TODO - Evaluate all of my created error types.  I should probably: "impl std::error::Error for ... {}" them

#[derive(Debug)]
pub struct IllegalStringError;
impl Display for IllegalStringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "IllegalStringError")
    }
}
impl std::error::Error for IllegalStringError {}
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum JokerId {
    A,
    B,
}

impl Display for JokerId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            JokerId::A => write!(f, "A"),
            JokerId::B => write!(f, "B"),
        }
    }
}

impl FromStr for JokerId {
    type Err = IllegalStringError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(JokerId::A),
            "B" => Ok(JokerId::B),
            _ => Err(IllegalStringError),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Suit::Club => write!(f, "C"),
            Suit::Diamond => write!(f, "D"),
            Suit::Heart => write!(f, "H"),
            Suit::Spade => write!(f, "S"),
        }
    }
}

impl FromStr for Suit {
    type Err = IllegalStringError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Suit::Club),
            "D" => Ok(Suit::Diamond),
            "H" => Ok(Suit::Heart),
            "S" => Ok(Suit::Spade),
            _ => Err(IllegalStringError),
        }
    }
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

pub type DefCardValue = BoundedU8<1, 54>; // default card value

static DEFAULT_VALUES: OnceCell<HashMap<Card, DefCardValue>> = OnceCell::new();

impl Card {
    /// Obtains the default card value
    /// Values are assigned 1 - 54 in new deck order. New deck order
    /// is: Ace-King hearts, Ace-King Clubs, King-Ace Diamonds, King-Ace Spades,
    /// Joker A, Joker B.
    ///
    /// # Examples
    /// ```
    /// use card_play::{Card, Suit, DefCardValue};
    /// let ah = &Card::Ace(Suit::Heart);
    /// let ah_val = ah.default_value();
    /// assert_eq!(ah_val, DefCardValue::new(1).unwrap());
    /// ```
    pub fn default_value(&self) -> DefCardValue {
        // can panic if value table init code broken - not all cards included
        *DEFAULT_VALUES
            .get_or_init(Cards::default_value_init)
            .get(self)
            .unwrap()
    }

    /// Obtains the next default card value in the new deck order sequence
    /// Values are assigned 1 - 54 in new deck order. New deck order
    /// is: Ace-King hearts, Ace-King Clubs, King-Ace Diamonds, King-Ace Spades,
    /// Joker A, Joker B.  Note that next value wraps around the end of the
    /// deck which is identified by the number of Jokers to include in the
    /// determination
    ///
    /// # Examples
    /// ```
    /// use card_play::{Card, Suit, DefCardValue, JokersPerDeck};
    /// let ah = &Card::Two(Suit::Club);
    /// let ah_val = ah.next_def_val_in_sequence(JokersPerDeck::new(2).unwrap());
    /// assert_eq!(ah_val, DefCardValue::new(16).unwrap());
    /// ```
    pub fn next_def_val_in_sequence(&self, jokers_per_deck: JokersPerDeck) -> DefCardValue {
        let last_val_in_new_deck = match i32::from(jokers_per_deck) {
            0 => Card::Ace(Suit::Spade).default_value(),
            1 => Card::Joker(JokerId::A).default_value(),
            2 => Card::Joker(JokerId::B).default_value(),
            _ => Card::Ace(Suit::Spade).default_value(),
        };
        // can panic if value table or value bounds code broken
        if u8::from(self.default_value()) < u8::from(last_val_in_new_deck) {
            DefCardValue::new(u8::from(self.default_value()) + 1).unwrap()
        } else {
            // to handle multiple decks, we allow wrap-around 54->1
            // which means a reversed new deck will find one sequence of two values
            // consisting of the first (joker with value of 54) then the last card
            // (an Ace of Hearts with a value of 1)
            DefCardValue::new(1).unwrap()
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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

impl FromStr for Card {
    type Err = IllegalStringError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(IllegalStringError);
        }
        // Can panic if previous/next line broken
        if s.chars().nth(0).unwrap() == 'F' {
            // Can panic if length check line or next line broken
            let id = JokerId::from_str(&s[1..=1])?;
            Ok(Card::Joker(id))
        } else {
            // Can panic if length check code or next line code broken
            let suit = match s.chars().nth(1).unwrap() {
                'S' => Suit::Spade,
                'H' => Suit::Heart,
                'D' => Suit::Diamond,
                'C' => Suit::Club,
                _ => return Err(IllegalStringError),
            };
            // Can panic if length check line or next line broken
            match s.chars().nth(0).unwrap() {
                'A' => Ok(Card::Ace(suit)),
                '2' => Ok(Card::Two(suit)),
                '3' => Ok(Card::Three(suit)),
                '4' => Ok(Card::Four(suit)),
                '5' => Ok(Card::Five(suit)),
                '6' => Ok(Card::Six(suit)),
                '7' => Ok(Card::Seven(suit)),
                '8' => Ok(Card::Eight(suit)),
                '9' => Ok(Card::Nine(suit)),
                'T' => Ok(Card::Ten(suit)),
                'J' => Ok(Card::Jack(suit)),
                'Q' => Ok(Card::Queen(suit)),
                'K' => Ok(Card::King(suit)),
                _ => Err(IllegalStringError),
            }
        }
    }
}

pub type NoiseLevel = BoundedU8<0, 10>;
pub type JokersPerDeck = BoundedU8<0, 2>;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Cards(pub Vec<Card>);

// new deck order (per above):
// hearts A, 2-K, clubs A, 2-K, Diamonds K-2, A, Spades K-2, A, Joker A, Joker B
impl Cards {
    /// Create a new set of cards in standard new-deck (Bicycle, USPCC std. etc.)
    /// order, top down, card faces down, Ace-King hearts, Ace-King Clubs, King-Ace Diamonds,
    /// King-Ace Spades, Joker A, Joker B.  Number of decks to be included and number of Jokers
    /// per deck must be specified.  Note that when standard order is discussed in
    /// the literature as top down, faces up, the order starts with Joker 1, Joker 2, Ace Spades,
    /// etc. (i.e. Joker A is Joker 2, Joker B is Joker 1). Something to keep in mind.
    ///
    /// # Examples
    /// ```
    /// use card_play::{Card, Cards, JokerId, JokersPerDeck};
    /// let new_deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
    /// assert_eq!(new_deck.len(), 54);
    /// assert_eq!(*new_deck.look_at(53).unwrap(), Card::Joker(JokerId::B));
    /// ```
    pub fn new(count: usize, jokers_cnt: JokersPerDeck) -> Cards {
        if count == 0 {
            return Cards(vec![]);
        };

        let mut deck = NEW_DECK_ARR[..=(51 + usize::from(jokers_cnt))].to_vec();

        if count > 1 {
            let mut additional: Vec<Card> = Vec::new();
            for _ in 2..count {
                for card in deck.iter() {
                    additional.push(*card);
                }
            }
            deck.append(&mut additional);
        }

        Cards(deck)
    }

    /// divide a card stack into two stacks with the division before the
    /// card specified by the index.  Put another way the length of the
    /// resulting top stack is equal to the index and the card identified
    /// by the index is the first card of the bottom stack
    ///
    /// # Examples
    /// ```
    /// use card_play::{Card, Cards, JokersPerDeck, Suit, TwoStacks};
    /// let new_deck = Cards::new(1, JokersPerDeck::new(0).unwrap());
    /// let TwoStacks(top, bottom) = new_deck.cut(52/2);
    /// assert_eq!(top.len(), bottom.len());
    /// assert_eq!(*bottom.look_at(0).unwrap(), Card::King(Suit::Diamond));
    /// ```
    pub fn cut(mut self, index: usize) -> TwoStacks {
        if index >= self.0.len() {
            return TwoStacks(self, Cards(vec![]));
        }
        let bottom = Cards(self.0.split_off(index));
        TwoStacks(self, bottom)
    }

    /// divide a stack into two stacks with the cut point
    /// random based on a normal distribution (mean at the
    /// half point) as follows:
    /// noise == 0 => exact cut after first half (even count) or
    /// 50/50 chance of the middle card (odd count) in the first or
    /// second half of the cut.
    /// for noise in 1 - 10 (inclusive) cut location is a normal
    /// distribution with mean at the center point and Variance approximately
    /// smoothly varying from 1 to the number of cards (i.e. Standard Deviation
    /// = 1 + (NoiseLevel - 1) * (Sqrt(number of cards))/9)
    /// the cut point is the index of the card before which we will cut -
    /// A cut point of 0 means the whole goes after the cut
    ///
    /// # Examples
    /// ```
    /// use card_play::{Cards, JokersPerDeck, TwoStacks, NoiseLevel};
    /// let new_deck = Cards::new(1, JokersPerDeck::new(0).unwrap());
    /// let new_deck_len = new_deck.len();
    /// let TwoStacks(top, bottom) = new_deck.cut_with_noise(NoiseLevel::new(5).unwrap());
    /// assert_eq!(top.len() + bottom.len(), new_deck_len);
    /// ```
    pub fn cut_with_noise(self, noise: NoiseLevel) -> TwoStacks {
        if noise == NoiseLevel::new(0).unwrap() {
            let count = self.0.len();
            self.cut(count / 2)
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

    /// perform riffle_count shuffles with a settable level of randomness in both the
    /// cut point and merge via a NoiseLevel parameter
    ///
    /// # Examples
    /// ```
    /// use card_play::{Cards, JokersPerDeck, NoiseLevel};
    /// let mut deck = Cards::new(1, JokersPerDeck::new(0).unwrap());
    /// let ref_deck = deck.clone();
    /// deck.shuffle(1, NoiseLevel::new(5).unwrap());
    /// assert_ne!(deck, ref_deck);
    /// ```
    pub fn shuffle(&mut self, riffle_count: usize, noise: NoiseLevel) {
        for _ in 0..riffle_count {
            // if shuffle noise is off (i.e. 0) use a "perfect" IN merge.
            // A perfect in merge of 52 cards should return deck to its original state after
            // 52 shuffles (whereas it only takes 8 perfect OUT shuffles to do so)
            let m_type = match u8::from(noise) {
                0 => MergeType::IN,
                _ => MergeType::RANDOM,
            };
            *self = self.clone().cut_with_noise(noise).merge(m_type);
        }
    }

    /// perform Fisher-Yates randomization (pick cards at random from origin deck to create
    /// destination deck) as a shuffle
    ///
    /// # Examples
    /// ```
    /// use card_play::{Cards, JokersPerDeck};
    /// let mut deck = Cards::new(1, JokersPerDeck::new(0).unwrap());
    /// let ref_deck = deck.clone();
    /// deck.shuffle_fy();
    /// assert_ne!(deck, ref_deck);
    /// ```
    pub fn shuffle_fy(&mut self) {
        // Fisher-Yates algo from Wikipedia
        let mut rng = rand::thread_rng();
        let n = self.0.len();
        for i in 0..(n - 2) {
            self.0.swap(i, rng.gen_range(i..n));
        }
    }

    /// perform perfect "in" shuffle (not random, original deck of even length, will reappear
    /// after shuffle count equal to the number of cards.)  With an "in" shuffle, the bottom card
    /// of the resulting stack is that which was on the bottom of the "top" stack.
    ///
    /// # Examples
    /// ```
    /// use card_play::{Cards, JokersPerDeck};
    /// let mut deck = Cards::new(1, JokersPerDeck::new(0).unwrap());
    /// let ref_deck = deck.clone();
    /// deck.in_shuffle(1);
    /// assert_ne!(deck, ref_deck);
    /// deck.in_shuffle(51);
    /// assert_eq!(deck, ref_deck);
    /// ```
    pub fn in_shuffle(&mut self, riffle_count: usize) {
        for _ in 0..riffle_count {
            // A perfect in merge of 52 cards should return deck to its original state after
            // 52 shuffles (whereas it only takes 8 perfect OUT shuffles to do so)
            *self = self.clone().cut(self.0.len() / 2).merge(MergeType::IN);
        }
    }
    /// perform perfect "out" shuffle (not random, original 52 card deck, will reappear
    /// after shuffle count equal to the number of cards.)  Note:  the bottom card of the resulting
    /// stack is that which was on the bottom of the bottom stack.
    ///
    /// # Examples
    /// ```
    /// use card_play::{Cards, JokersPerDeck};
    /// let mut deck = Cards::new(1, JokersPerDeck::new(0).unwrap());
    /// let ref_deck = deck.clone();
    /// deck.out_shuffle(1);
    /// assert_ne!(deck, ref_deck);
    /// deck.out_shuffle(7);
    /// assert_eq!(deck, ref_deck);
    /// ```
    pub fn out_shuffle(&mut self, riffle_count: usize) {
        for _ in 0..riffle_count {
            // A perfect in merge of 52 cards should return deck to its original state after
            // 52 shuffles (whereas it only takes 8 perfect OUT shuffles to do so)
            *self = self.clone().cut(self.0.len() / 2).merge(MergeType::OUT);
        }
    }
    // required to init values for *all* possible cards
    fn default_value_init() -> HashMap<Card, DefCardValue> {
        let mut values = HashMap::new();
        // following can panic if next line broken - illegal value for JokersPerDeck
        let new_deck = Cards::new(1, JokersPerDeck::new(2).unwrap()); // new deck w/ Joker -> 54 cards
        for (i, card) in new_deck.0.iter().enumerate() {
            // can panic if next line broken - illegal card value.
            values.insert(*card, DefCardValue::new((i + 1) as u8).unwrap()); // values not zero based
        }
        values
    }

    /// Rising sequence count metric (from numerous sources, e.g. "Shuffling Study.pdf" Caedmon)
    /// with modifications for jokers and multiple decks.  Note, max length rising sequences
    /// include "sequences" of just a single value (not obvious why but that's the way they are
    /// counted in the literature...) and the closer the value is to the deck size/2 seems to be the
    /// actual metric (bell curve and all that):
    /// //https://math.stackexchange.com/questions/4354898/how-can-you-measure-how-shuffled-a-deck-of-cards-is
    /// https://drive.google.com/file/d/1EoJhtHAO5iFjikkH35KDVrmmJQpXVb5q/view?usp=sharing
    /// Note that my adaptation for the inclusion of one or more jokers per deck and the use of
    /// multiple decks will lead to different values for the same level of shuffling of the one-deck
    /// no joker case (which can be checked by comparing riffle shuffling with Fisher-Yates).
    /// Presence of one or Jokers per deck determined by modulo 52 calculation.
    ///
    /// # Examples
    /// ```
    /// use card_play::{Cards, JokersPerDeck, NoiseLevel};
    /// let mut deck = Cards::new(1, JokersPerDeck::new(0).unwrap());
    /// assert_eq!(deck.shuffle_rs_metric(), 1);
    /// deck.shuffle(1, NoiseLevel::new(0).unwrap());
    /// assert_eq!(deck.shuffle_rs_metric(), 2);
    /// deck.shuffle_fy();
    /// // the following will fail a fraction of the time
    /// // !(deck.shuffle_rs_metric() > 23 && deck.shuffle_rs_metric() < 29);
    /// ```
    pub fn shuffle_rs_metric(&self) -> usize {
        let deck_cnt = self.0.len() / 52;
        let mut jokers_per_deck = (self.0.len() % 52) / deck_cnt;
        if jokers_per_deck > 2 {
            jokers_per_deck = 0; // if non-complete decks being used - assume no jokers
        }
        // can panic if bounds limiting code above broken
        let jokers_per_deck = JokersPerDeck::new(jokers_per_deck as u8).unwrap();
        let mut n: usize = 0;
        let mut in_sequence = vec![false; self.0.len()];
        for (i, start) in self.0[0..self.0.len() - 1].iter().enumerate() {
            if !in_sequence[i] {
                in_sequence[i] = true;
                n += 1;
            }
            let mut this = start;
            for (k, candidate_card) in self.0[i + 1..].iter().enumerate() {
                if usize::from((*candidate_card).default_value())
                    == usize::from((*this).next_def_val_in_sequence(jokers_per_deck))
                    && !in_sequence[i + 1 + k]
                {
                    in_sequence[i + 1 + k] = true;
                    this = candidate_card;
                }
            }
        }
        // Need to also check the very last one as it might be a sequence of one
        if !in_sequence[self.0.len() - 1] {
            in_sequence[self.0.len() - 1] = true;
            n += 1;
        }
        n
    }

    /// reverse the order of Cards
    ///
    /// # Examples
    /// ```
    /// use card_play::{Card, Suit, Cards, JokersPerDeck, JokerId};
    /// let mut deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
    /// assert_eq!(*deck.look_at(0).unwrap(), Card::Ace(Suit::Heart));
    /// deck.reverse();
    /// assert_eq!(*deck.look_at(0).unwrap(), Card::Joker(JokerId::B));
    /// ```
    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    /// reposition a specified index of a specified card by a specified number of places
    /// (i.e. in a multi-deck stack, the second occurrence of the six of hearts would have index 1)
    /// If the card displacement wraps around the end of the deck, the move from one end to the
    /// other counts as a one position change.
    /// returns true if card found in stack, false otherwise.
    ///
    /// # Examples
    /// ```
    /// use card_play::{Card, Suit, Cards, JokersPerDeck};
    /// let mut deck = Cards::new(1, JokersPerDeck::new(0).unwrap());
    /// assert_eq!(*deck.look_at(5).unwrap(), Card::Six(Suit::Heart));
    /// assert!(deck.move_card(Card::Six(Suit::Heart), 0, -5));
    /// assert_eq!(*deck.look_at(0).unwrap(), Card::Six(Suit::Heart));
    /// assert!(deck.move_card(Card::Six(Suit::Heart), 0, -1));
    /// assert_eq!(*deck.look_at(51).unwrap(), Card::Six(Suit::Heart));
    /// assert!(deck.move_card(Card::Six(Suit::Heart), 0, 1));
    /// assert_eq!(*deck.look_at(0).unwrap(), Card::Six(Suit::Heart));
    /// ```
    pub fn move_card(&mut self, card: Card, match_index: usize, position_change: isize) -> bool {
        let Some(position_start) = self
            .0
            .iter()
            .enumerate()
            // linter likes the following two commented-out lines better than my filter_map line
            // .filter(|(_idx, r)| (**r == card))
            // .map(|(idx, _r)| idx)
            .filter_map(|(idx, r)| (*r == card).then(|| idx))
            .nth(match_index)
        else {
            return false;
        };
        let position_end =
            (position_start as isize + position_change).rem_euclid(self.0.len() as isize) as usize;

        let card = self.0.remove(position_start);

        // ensure we are still in the range vec insert requires (corner case at end of deck)
        // TODO - position_end = position_end.rem_euclid(self.0.len() + 1);

        self.0.insert(position_end, card);

        true
    }

    /// reposition a specified index of a specified card by a specified number of places
    /// (i.e. in a multi-deck stack, the first occurrence of the size of the card, say, six of
    /// hearts, would have an index of 0, the second occurrence would have index 1)
    /// If the card displacement wraps around the end of the deck, the move from one end to the
    /// other DOES NOT count as a one position change.
    /// returns true if card found in stack, false otherwise.
    ///
    /// # Examples
    /// ```
    /// use card_play::{Card, Suit, Cards, JokersPerDeck};
    /// let mut deck = Cards::new(1, JokersPerDeck::new(0).unwrap());
    /// assert_eq!(*deck.look_at(5).unwrap(), Card::Six(Suit::Heart));
    /// assert!(deck.move_card_circular(Card::Six(Suit::Heart), 0, -6));
    /// assert_eq!(*deck.look_at(50).unwrap(), Card::Six(Suit::Heart));
    /// ```
    pub fn move_card_circular(
        &mut self,
        card: Card,
        match_index: usize,
        position_change: isize,
    ) -> bool {
        let Some(position_start) = self
            .0
            .iter()
            .enumerate()
            .filter_map(|(idx, r)| (*r == card).then(|| idx))
            .nth(match_index)
        else {
            return false;
        };
        let mut position_end =
            (position_start as isize + position_change).rem_euclid(self.0.len() as isize) as usize;

        // perform wrap around adjustment (i.e. as if cards are in a circle, not a stack)
        // if position change is positive and position_end is less than position start, we need to
        // add one (since there is no card to skip over between the last and first in a stack as we
        // wrap around).  Similarly, if the position change is negative and position_end is greater
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

    /// get the index for a cards as an Option<usize>.  None if not found
    ///
    /// # Examples
    /// ```
    /// use card_play::{Card, Suit, Cards, JokersPerDeck};
    /// let deck = Cards::new(1, JokersPerDeck::new(0).unwrap());
    /// assert_eq!(deck.find(Card::Five(Suit::Heart)).unwrap(), 4);
    /// ```
    pub fn find(&self, card: Card) -> Option<usize> {
        self.0.iter().position(|r| *r == card)
    }

    /// draw count cards
    /// Result Err a string indicating more cards were requested
    /// than are present
    ///
    /// # Examples
    /// ```
    /// use card_play::{Cards, JokersPerDeck};
    /// let mut deck = Cards::new(1, JokersPerDeck::new(0).unwrap());
    /// let hand = deck.draw_count(5).unwrap();
    /// assert_eq!(hand.len(), 5);
    /// ```
    pub fn draw_count(&mut self, count: usize) -> Result<Cards, &'static str> {
        if count > self.0.len() {
            return Err("Can not draw more than are available");
        }
        Ok(Cards(self.0.drain(0..count).collect()))
    }

    /// draw all cards preceding that of the card specified.
    ///
    /// # Examples
    /// ```
    /// use card_play::{Card, JokerId, Cards, JokersPerDeck};
    /// let mut deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
    /// let all_but_one = deck.draw_till(Card::Joker(JokerId::B)).unwrap();
    /// assert_eq!(all_but_one.len(), 53);
    /// assert_eq!(*deck.look_at(0).unwrap(), Card::Joker(JokerId::B));
    /// ```
    pub fn draw_till(&mut self, card: Card) -> Option<Cards> {
        let count = self.0.iter().position(|r| *r == card);
        count.map(|count| self.draw_count(count).unwrap())
    }

    /// append a stack to the end
    ///
    /// # Examples
    /// ```
    /// use card_play::{Cards, JokersPerDeck};
    /// let mut deck = Cards::new(1, JokersPerDeck::new(0).unwrap());
    /// let mut again = deck.clone();
    /// deck.append(again);
    /// assert_eq!(deck.len(), 2 * 52);
    /// ```
    pub fn append(&mut self, mut cards: Cards) {
        self.0.append(&mut cards.0);
    }

    /// obtain a reference Result to a card at a give index.  Error string if index out of range
    ///
    /// # Examples
    /// ```
    /// use card_play::{Card, Cards, JokerId, JokersPerDeck};
    /// let deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
    /// let card = deck.look_at(52).unwrap();
    /// assert_eq!(*card, Card::Joker(JokerId::A));
    /// ```
    pub fn look_at(&self, index: usize) -> Result<&Card, &'static str> {
        if index >= self.0.len() {
            return Err("Index beyond end of Cards");
        }
        Ok(&self.0[index])
    }

    /// Obtain a representation of the Card sequence as a vector of their default values
    /// (Ace Hearts == 1 through Joker B == 54)
    ///
    /// # Examples
    /// ```
    /// use card_play::{Cards, JokersPerDeck};
    /// let deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
    /// let deck_values = deck.by_def_raw_values();
    /// assert_eq!(deck_values[0..5], vec![1, 2, 3, 4, 5]);
    /// ```
    pub fn by_def_raw_values(&self) -> Vec<u8> {
        let values: Vec<u8> = self
            .0
            .iter()
            .map(|c| u8::from((*c).default_value()))
            .collect();
        values
    }

    /// get the length
    ///
    /// # Examples
    /// ```
    /// use card_play::{Cards, JokersPerDeck};
    /// let deck = Cards::new(1, JokersPerDeck::new(1).unwrap());
    /// assert_eq!(deck.len(), 53);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for Cards {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut it = self.0.iter().peekable();
        while let Some(card) = it.next() {
            write!(f, "{}", card)?;
            if it.peek().is_some() {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

impl FromStr for Cards {
    type Err = IllegalStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards: Cards = Cards(vec![]);
        for card_str in s.split(' ') {
            cards.0.push(Card::from_str(card_str)?);
        }
        Ok(cards)
    }
}

pub struct TwoStacks(pub Cards, pub Cards);

#[derive(PartialEq)]
pub enum MergeType {
    IN,
    OUT,
    RANDOM,
}
impl TwoStacks {
    /// combine the two stacks in TwoStacks into one Cards stack by nominally alternating
    /// from the stacks starting at the bottom as in a riffle shuffle but with one of three
    /// different techniques as specified by the MergeType.
    /// MergeType::RANDOM uses effectively a coin flip to determine which stack goes next
    /// MergeType::IN starts at the bottom of the top stack and alternates between stacks from then on.
    /// MergeType::Out starts at the bottom of the bottom stack and alternates between stacks from then on.
    /// While a perfect faro shuffle (IN or OUT) assumes an equal number of cards in each stack
    /// the merge method will just keep pulling from the stack that as cards if the other stack has
    /// been depleted.
    ///
    /// # Examples
    /// ```
    /// use card_play::{Card, Suit, Cards, JokersPerDeck, MergeType};
    /// let deck = Cards::new(1, JokersPerDeck::new(0).unwrap());
    /// let two_stacks = deck.cut(26);
    /// let deck = two_stacks.merge(MergeType::IN);
    /// assert_eq!(*deck.look_at(51).unwrap(), Card::King(Suit::Club));
    /// assert_eq!(*deck.look_at(50).unwrap(), Card::Ace(Suit::Spade));
    /// ```
    pub fn merge(self, m_type: MergeType) -> Cards {
        let TwoStacks(mut top, mut bottom) = self;
        let mut cards = Cards::default();
        let mut rng = rand::thread_rng();
        for i in 0..(top.0.len() + bottom.0.len()) {
            let first_try: &mut Vec<Card>;
            let then_try: &mut Vec<Card>;
            // Reminder - we are popping from the bottom of the stacks and later reversing
            // So an IN merge will result in the last card of the top stack on the bottom
            if m_type == MergeType::IN && (i % 2) == 0
                || m_type == MergeType::OUT && (i % 2) == 1
                || m_type == MergeType::RANDOM && rng.gen()
            {
                first_try = &mut top.0;
                then_try = &mut bottom.0;
            } else {
                first_try = &mut bottom.0;
                then_try = &mut top.0;
            }

            if let Some(card) = first_try.pop() {
                cards.0.push(card)
            } else if let Some(card) = then_try.pop() {
                cards.0.push(card)
            } else {
                panic!("Must have loop counter wrong!");
            }
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
        let deck = Cards::new(1, JokersPerDeck::new(2).expect("new JokersPerDeck failed"));
        assert_eq!(deck.0.len(), 54, "new deck with jokers wrong length");
        for (i, card) in deck.0.iter().enumerate() {
            assert_eq!(
                *card, NEW_DECK_ARR[i],
                "new deck cards don't match ref array"
            );
        }
    }

    #[test]
    fn test_raw_and_default_values() {
        let deck = Cards::new(1, JokersPerDeck::new(2).expect("new JokersPerDeck failed"));
        for (i, value) in deck.by_def_raw_values().iter().enumerate() {
            assert_eq!(usize::from(*value), i + 1, "broken raw value function");
        }
    }

    #[test]
    fn test_rs_shuffle_metric_sanity() {
        let mut deck = Cards::new(1, JokersPerDeck::new(2).expect("new JokersPerDeck failed"));
        assert_eq!(
            deck.shuffle_rs_metric(),
            1,
            "New deck did not net rs_metric of 1"
        );
        deck.reverse();
        assert_eq!(
            deck.shuffle_rs_metric(),
            53,
            "New reversed deck did not get rs_metric of 53"
        );
        deck.reverse();
        deck.shuffle(1, NoiseLevel::new(0).expect("new NoiseLevel failed"));
        assert_eq!(
            deck.shuffle_rs_metric(),
            2,
            "New deck after one noiseless shuffle did not get rs_metric of 2"
        );
        deck.reverse();
        assert_eq!(
            deck.shuffle_rs_metric(),
            52,
            "New noiseless shuffle, reversed did not get rs_metric of 52"
        );
    }

    #[test]
    fn test_rs_metric_and_fs_shuffle_statistics() {
        const ITER_COUNT: usize = 1000;
        let mut metrics = [0usize; ITER_COUNT];
        let mut deck_size: usize = 0;
        for metric in metrics.iter_mut() {
            let mut deck = Cards::new(1, JokersPerDeck::new(2).expect("new JokersPerDeck"));
            deck_size = deck.0.len();
            assert_eq!(
                deck_size % 2,
                0,
                "code assumption of even sized deck is broken"
            );
            deck.shuffle_fy();
            *metric = deck.shuffle_rs_metric();
        }
        let sum = metrics.iter().sum::<usize>() as f64;
        let mean = sum / ITER_COUNT as f64;
        // round to nearest whole number
        assert_eq!((mean + 0.5) as usize, deck_size / 2);
    }

    #[test]
    fn test_riffle_shuffles_and_fs_shuffle_statistics() {
        const ITER_COUNT: usize = 1000;
        let mut metrics = [0usize; ITER_COUNT];
        let mut deck_size: usize = 0;
        for metric in metrics.iter_mut() {
            let mut deck = Cards::new(1, JokersPerDeck::new(2).expect("new JokersPerDeck failed"));
            deck_size = deck.0.len();
            assert_eq!(
                deck_size % 2,
                0,
                "code assumption of even sized deck is broken"
            );
            deck.shuffle(12, NoiseLevel::new(5).expect("new NoiseLevel failed"));
            *metric = deck.shuffle_rs_metric();
        }
        let sum = metrics.iter().sum::<usize>() as f64;
        let mean = sum / ITER_COUNT as f64;
        // round to nearest whole number
        assert_eq!((mean + 0.5) as usize, deck_size / 2);
    }

    #[test]
    fn test_in_out_shuffles() {
        let mut deck = Cards::new(1, JokersPerDeck::new(0).expect("new JokersPerDeck failed"));
        let reference_deck =
            Cards::new(1, JokersPerDeck::new(0).expect("new JokersPerDeck failed"));
        let deck_size = deck.0.len();
        assert_eq!(
            deck_size % 2,
            0,
            "code assumption of even sized deck is broken"
        );
        deck.in_shuffle(8);
        assert_ne!(deck, reference_deck);
        deck.in_shuffle(deck_size - 8);
        assert_eq!(deck, reference_deck);
        let mut deck = Cards::new(1, JokersPerDeck::new(0).expect("new JokersPerDeck failed"));
        deck.out_shuffle(8);
        assert_eq!(deck, reference_deck);
    }

    #[test]
    fn test_cards_from_str() {
        let card_str = "AC QH FA FB";
        let cards = Cards::from_str(card_str);
        let cards = cards.expect("Illegal String");
        let new_card_str = cards.to_string();
        assert_eq!(new_card_str, "AC QH FA FB");
    }
}
