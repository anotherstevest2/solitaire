use std::fmt;
use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;
use std::string::ParseError;
use bounded_integer::BoundedU8;
use rand_distr::{Normal, Distribution};
use rand::Rng;
use once_cell::sync::OnceCell;

// TODO - import and refactor to use anyhow for error handling;
// TODO - define deref trait so that we don't need the .0 in cards.0.len() etc.
//        Umm... Cancel the above, for predictability reasons (per the API guidance)
//        deref should *only* by defined for smart pointers.  Instead, the desired
//        methods (len, push, pop etc...) should be defined for the newtype
//        seems non-ideal... or, just keep the destructuring/.0 bit in the usage.
//        so: in the crate (where visible via an api), implement all the functions
//        whereas internally, keep the .0 for the new types which don't appear in
//        the api.
//        Another Idea:  Create a trait for NewtypeVec which unwraps the newtype to make
//        All of the methods that work on a vector, work on the newtype.
// TODO - Need to better understand how to do math with bounded values.  At this point it
//        appears that some/all math that includes bounded values gets checking whereas
//        if you extract raw values, do math, create new bounded values it works as expected.
//        the issue might be just if you are saving to an existing bounded value - and maybe
//        this is to ensure no out-of-bounds intermediate values but, as it is, I'm clearly
//        not using it right.  and it might be best to make a "raw_value" method for all of them
//        and always do the math with a raw value and create a new bounded value with the result.
//

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
        let mut position_end = (position_start as isize + position_change).rem_euclid(self.0.len() as isize) as usize;

        let card = self.0.remove(position_start);

        // adjust for vector index offset caused by removing the card
        if position_end > position_start {
            position_end += 1;
        }

        // ensure we are still in the range vec insert requires (corner case at end of deck)
        position_end = position_end.rem_euclid(self.0.len() + 1) as usize;

        self.0.insert(position_end, card);

        Ok(self.clone())
    }

    fn move_card_circular(&mut self, card: Card, position_change: isize) -> Result<Cards, String> {
        let position_start = self.0.iter().position(|r| *r == card).ok_or("Card not found")?;
        let mut position_end = (position_start as isize + position_change).rem_euclid(self.0.len() as isize) as usize;

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

fn letter_into_value(ul: &UpperLetter) -> LetterValue {
    LetterValue::new(u8::from(*ul) - 64).unwrap()
}

fn value_into_letter(lv: &LetterValue) -> UpperLetter {
    UpperLetter::new(u8::from(*lv) + 64).unwrap()
}

type LetterValue = BoundedU8<1, 26>;


type CardValue = BoundedU8<1, 53>;

fn card_val_into_position(cv: &CardValue) -> CardPosition {
    CardPosition::new(u8::from(*cv)).unwrap()
}

fn card_val_into_let_val(cv: CardValue) -> LetterValue {
    // need to convert sum to zero based (-1) before modulo and back to one based (+1) after
    LetterValue::new((u8::from(cv) - 1) % 26 + 1).unwrap()
}

type CardPosition = BoundedU8<1, 54>;

#[derive(Debug)]
struct PlainText (Vec<UpperLetter>);

impl PlainText {
    fn new() -> PlainText {
        PlainText(Vec::new())
    }

    fn pt_to_string(&self) -> String {
        let mut s = String::new();
        for c in self.0.iter() {
            s.push(u8::from(*c) as char);
        }
        s
    }
}

impl FromStr for PlainText {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pt = PlainText(Vec::new());
        for letter in s.bytes() {
            if let l = UpperLetter::new(letter).unwrap() {
                pt.0.push(l);
            }
        }
        Ok(pt)
    }
}

#[derive(Debug)]
struct CypherText (Vec<UpperLetter>);
impl CypherText {
    fn new() -> CypherText {
        CypherText(Vec::new())
    }

    fn ct_to_string(&self) -> String {
        let mut s = String::new();
        for c in self.0.iter() {
            s.push(u8::from(*c) as char);
        }
        s
    }
}

impl FromStr for CypherText {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ct = CypherText(Vec::new());
        for letter in s.bytes() {
            if let l = UpperLetter::new(letter).unwrap() {
                ct.0.push(l);
            }
        }
        Ok(ct)
    }
}

#[derive(Debug)]
struct Passphrase (Vec<UpperLetter>);

impl Passphrase {
    fn iter(&self) -> std::slice::Iter<'_, UpperLetter> {
        self.0.iter()
    }

    fn pp_to_string(&self) -> String {
        let mut s = String::new();
        for c in self.0.iter() {
            s.push(u8::from(*c) as char);
        }
        s
    }
}

impl FromStr for Passphrase {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pp = Passphrase(Vec::new());
        for letter in s.to_uppercase().bytes() {
            if let l = UpperLetter::new(letter).unwrap() {
                pp.0.push(l);
            }
        }
        Ok(pp)
    }
}

struct KeyStream (Vec<LetterValue>);

impl KeyStream {
    fn new() -> KeyStream {
        KeyStream(Vec::new())
    }
    fn ks_to_string(&self) -> String {
        let mut s = String::new();
        for c in self.0.iter() {
            s.push_str(&(u8::from(*c).to_string()));
            s.push_str(" ");
        }
        s
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
    // add the last joker (order is A then B) with same (53) value as the other one
    values.insert(Card::Joker(JokerId::B), CardValue::new(53u8).unwrap());
    values
}

fn main() {
    VALUES.get_or_init(|| {value_init()});

    fn next_deck_state(mut key_deck: Cards) -> Cards {
        // A Joker move
        let mut key_deck = key_deck.move_card_circular(Card::Joker(JokerId::A), 1).unwrap();
        // B Joker move
        key_deck = key_deck.move_card_circular(Card::Joker(JokerId::B), 2).unwrap();

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
        let TwoStacks(top, mut bottom) = key_deck
            .cut((*bottom_card_value).into());
        let bottom_card = bottom.0
            .pop()
            .unwrap();
        let key_deck = bottom
            .append(top)
            .append(Cards(vec!(bottom_card)));
        key_deck.clone()
    }

    fn key_deck_from_passphrase(passphrase: &Passphrase) -> Cards {
        let mut deck = Cards::new(DeckStyle::Jokers, 1);

        for letter in passphrase.iter() {
            deck = next_deck_state(deck);

            // count cut at passphrase letter value, maintain bottom card
            let letter_value = letter_into_value(letter);
            let TwoStacks(top, mut bottom) = deck
                .clone()
                .cut(letter_value.into());
            let bottom_card = bottom.0
                .pop()
                .unwrap();
            deck = bottom
                .append(top)
                .append(Cards(vec!(bottom_card)))
                .clone();
        }
        deck
    }

    fn get_key_stream(key_deck: Cards, key_length: usize) -> KeyStream {
        let mut key_deck = key_deck;
        let mut key_stream = KeyStream::new();
        // Ensure key length is a multiple of 5 (as is tradition) so the cypher text will be also
        let key_length= ((key_length + 4) / 5) * 5; // integer divide to drop remainder
        while key_stream.0.len() < key_length {
            key_deck = next_deck_state(key_deck);

            // Find output card, or Joker
            let top_card_value = &key_deck
                .look_at(0)
                .unwrap()
                .value();
            // hidden canceling adjustments: top_card_value [1..53] so subtract 1 to make it
            // an index range of [0..52] (i.e. so original 1 is pointing to first card)
            // and then add 1 to look at card *after* the one indexed
            // by the top card value for a net adjustment of 0
            let output_card_candidate_position = card_val_into_position(top_card_value);
            let output_card_candidate = &key_deck
                .look_at(output_card_candidate_position
                    .into())
                .unwrap();
            if  **output_card_candidate != Card::Joker(JokerId::A)
            && **output_card_candidate != Card::Joker(JokerId::B) {
                key_stream.0
                .push(card_val_into_let_val((*output_card_candidate).value()));
            }
        }
        key_stream
    }

    fn encrypt(pt: &PlainText, ks: &KeyStream) -> CypherText {
        if pt.0.len() > ks.0.len() {
            panic!("KeyStream not long enough");
        }

        let mut ct: CypherText = CypherText(vec!());
        for (i, k) in ks.0.iter().enumerate() {
            let pt_value = letter_into_value(&pt.0[i]);
            // need to convert sum to zero based (-1) before modulo and back to one based (+1) after
            ct.0.push(value_into_letter(&(LetterValue::new(((u8::from((pt_value)) + k - 1) % 26) + 1).unwrap())));
        }
        ct
    }

    fn decrypt(ct: &CypherText, ks: &KeyStream) -> PlainText {

        if ct.0.len() > ks.0.len() {
            panic!("KeyStream not long enough");
        }

        let mut pt: PlainText = PlainText(vec!());
        for (i, k) in ks.0.iter().enumerate() {
            let ct_value = letter_into_value(&ct.0[i]);
            pt.0.push(value_into_letter(&((ct_value - k).rem_euclid(26))));
        }
        pt
    }

    // let deck = Cards::new(DeckStyle::Jokers, 1);
    // println!("New deck: {deck}");
    // println!();
    // let TwoStacks(top, bottom) = deck.cut_with_noise(NoiseLevel::new(10).unwrap());
    // println!("After cut top: {top}");
    // println!();
    // println!("After cut bottom: {bottom}");
    // println!();
    // println!("Cut point: {}", top.0.len());
    // println!();
    // let deck = TwoStacks(top, bottom).merge();
    // println!("after first riffle:");
    // println!("{}", deck);
    // println!();
    // let mut deck = deck.shuffle(10, NoiseLevel::new(10).unwrap());
    // println!("after 10 more riffles:");
    // println!("{}", deck);
    // println!();

    // let mut new_deck = Cards::new(DeckStyle::Jokers, 1);


    // let mut spare_new_deck = new_deck.clone();
    //
    // let pt = PlainText::from_str("AAAAAAAAAAAAAAA").unwrap();
    // let ks = get_key_stream(new_deck, pt.0.len());
    // println!("key: <null key>, pt: {}, ks: {}", pt.pt_to_string(), ks.ks_to_string());
    //
    // let ct = encrypt(&pt, &ks);
    // println!("ct: {:?}", ct.ct_to_string());
    //

    // Sample output practice per Scheier

    // let mut key_deck = Cards::new(DeckStyle::Jokers, 1);
    // println!("new deck: {}", key_deck);
    //
    // let mut key_deck = key_deck.move_card_circular(Card::Joker(JokerId::A), 1).unwrap();
    // println!("after A joker move: \n{}", key_deck);
    //
    // key_deck = key_deck.move_card_circular(Card::Joker(JokerId::B), 2).unwrap();
    // println!("after B joker move: \n{}", key_deck);
    //
    // // Triple cut at Jokers (aka fools. fa, fb being fool A and fool B respectively)
    // // and swap top with bottom leaving Jokers in place
    // let mut above_fa = key_deck.draw_till(Card::Joker(JokerId::A)).unwrap();
    // if let Ok(above_both) = above_fa.draw_till(Card::Joker(JokerId::B)) {
    //     // Order is: above_both, FB above_fa, FA key_deck
    //     let fb = above_fa.draw_count(1).unwrap();
    //     let fa = key_deck.draw_count(1).unwrap();
    //     // swap top and bottom leaving fools in same relative positions
    //     key_deck.append(fb);
    //     key_deck.append(above_fa);
    //     key_deck.append(fa);
    //     key_deck.append(above_both);
    // } else if let Ok(mut above_fb) = key_deck.draw_till(Card::Joker(JokerId::B)) {
    //     // Order is: above_fa, FA above_fb, FB key_deck
    //     let fa = above_fb.draw_count(1).unwrap();
    //     let fb = key_deck.draw_count(1).unwrap();
    //     // swap top and bottom leaving fools in same relative positions
    //     key_deck.append(fa);
    //     key_deck.append(above_fb);
    //     key_deck.append(fb);
    //     key_deck.append(above_fa);
    // }
    // println!("after triple cut: \n{}", key_deck);
    //
    // let bottom_card_value = &key_deck.
    //     look_at(key_deck.0.len() - 1)
    //     .unwrap()
    //     .value();
    // println!("bottom card value {}", u8::from(*bottom_card_value));
    // let TwoStacks(top, mut bottom) = key_deck
    //     .cut((*bottom_card_value).into());
    // println!("top stack: {}", top);
    // println!("bottom stack: {}", bottom);
    // let bottom_card = bottom.0
    //     .pop()
    //     .unwrap();
    // let key_deck = bottom
    //     .append(top)
    //     .append(Cards(vec!(bottom_card)));
    // println!("after count cut: \n{}", key_deck);
    //
    // // Find output card, or Joker
    // let top_card_value = &key_deck
    //     .look_at(0)
    //     .unwrap()
    //     .value();
    // // hidden canceling adjustments: top_card_value [1..53] so subtract 1 to make it
    // // an index range of [0..52] (i.e. so original 1 is pointing to first card)
    // // and then add 1 to look at card *after* the one indexed
    // // by the top card value for a net adjustment of 0
    // let output_card_candidate_position = card_val_into_position(top_card_value);
    // let output_card_candidate = &key_deck
    //     .look_at(output_card_candidate_position
    //         .into())
    //     .unwrap();
    // println!("Output Card Candidate: {}", **output_card_candidate);
    //
    // // Repeat the whole cycle again:
    // println!("repeating the whole process again");
    // let mut key_deck = key_deck.move_card_circular(Card::Joker(JokerId::A), 1).unwrap();
    // println!("after A joker move: \n{}", key_deck);
    //
    // key_deck = key_deck.move_card_circular(Card::Joker(JokerId::B), 2).unwrap();
    // println!("after B joker move: \n{}", key_deck);
    //
    // // Triple cut at Jokers (aka fools. fa, fb being fool A and fool B respectively)
    // // and swap top with bottom leaving Jokers in place
    // let mut above_fa = key_deck.draw_till(Card::Joker(JokerId::A)).unwrap();
    // if let Ok(above_both) = above_fa.draw_till(Card::Joker(JokerId::B)) {
    //     // Order is: above_both, FB above_fa, FA key_deck
    //     let fb = above_fa.draw_count(1).unwrap();
    //     let fa = key_deck.draw_count(1).unwrap();
    //     // swap top and bottom leaving fools in same relative positions
    //     key_deck.append(fb);
    //     key_deck.append(above_fa);
    //     key_deck.append(fa);
    //     key_deck.append(above_both);
    // } else if let Ok(mut above_fb) = key_deck.draw_till(Card::Joker(JokerId::B)) {
    //     // Order is: above_fa, FA above_fb, FB key_deck
    //     let fa = above_fb.draw_count(1).unwrap();
    //     let fb = key_deck.draw_count(1).unwrap();
    //     // swap top and bottom leaving fools in same relative positions
    //     key_deck.append(fa);
    //     key_deck.append(above_fb);
    //     key_deck.append(fb);
    //     key_deck.append(above_fa);
    // }
    // println!("after triple cut: \n{}", key_deck);
    //
    // let bottom_card_value = &key_deck.
    //     look_at(key_deck.0.len() - 1)
    //     .unwrap()
    //     .value();
    // println!("bottom card value {}", u8::from(*bottom_card_value));
    // let TwoStacks(top, mut bottom) = key_deck
    //     .cut((*bottom_card_value).into());
    // println!("top stack: {}", top);
    // println!("bottom stack: {}", bottom);
    // let bottom_card = bottom.0
    //     .pop()
    //     .unwrap();
    // let key_deck = bottom
    //     .append(top)
    //     .append(Cards(vec!(bottom_card)));
    // println!("after count cut: \n{}", key_deck);
    //
    // // Find output card, or Joker
    // let top_card_value = &key_deck
    //     .look_at(0)
    //     .unwrap()
    //     .value();
    // // hidden canceling adjustments: top_card_value [1..53] so subtract 1 to make it
    // // an index range of [0..52] (i.e. so original 1 is pointing to first card)
    // // and then add 1 to look at card *after* the one indexed
    // // by the top card value for a net adjustment of 0
    // let output_card_candidate_position = card_val_into_position(top_card_value);
    // let output_card_candidate = &key_deck
    //     .look_at(output_card_candidate_position
    //         .into())
    //     .unwrap();
    // println!("Output Card Candidate: {}", **output_card_candidate);





    println!("First vector test:");
    let mut new_deck = Cards::new(DeckStyle::Jokers, 1);
    let pt = PlainText::from_str("AAAAAAAAAAAAAAA").unwrap();
    let ks = get_key_stream(new_deck, pt.0.len());
    let ct = encrypt(&pt, &ks);
    println!("key: <null key>, pt: {}, ks: {}", pt.pt_to_string(), ks.ks_to_string());
    println!("ct: {:?}", ct.ct_to_string());

    println!("Second vector test:");
    let mut new_deck = Cards::new(DeckStyle::Jokers, 1);
    let passphrase: Passphrase = Passphrase::from_str("f").unwrap();
    new_deck = key_deck_from_passphrase(&passphrase);
    let pt = PlainText::from_str("AAAAAAAAAAAAAAA").unwrap();
    let ks = get_key_stream(new_deck, pt.0.len());
    let ct = encrypt(&pt, &ks);
    println!("key: {}, pt: {}, ks: {}", passphrase.pp_to_string(), pt.pt_to_string(), ks.ks_to_string());
    println!("ct: {:?}", ct.ct_to_string());
}
