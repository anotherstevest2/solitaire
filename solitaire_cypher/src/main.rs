
// ----------------- TODO - Cards crate above, Solitaire Cypher Crate Below ----------------------
// When refactoring - consider doing newtype pattern on Bounded variables so that from can be
// defined.

use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;
use bounded_integer::BoundedU8;
use once_cell::sync::OnceCell;
use card_play::*;
use sdk::*;

// TODO - Rethink this being a trait - how to best support the user defining their own value table
//        For Card values...
trait Value {
    // (if used?) required to init values for all possible cards, values must be in range of CardValue
    fn value(&self) -> CardValue;
}

pub mod sdk;

type UpperLetter = BoundedU8<65, 90>;

fn letter_into_value(ul: &UpperLetter) -> LetterValue {
    // can panic if UpperLetter bounds code or next line broken
    LetterValue::new(u8::from(*ul) - 64).unwrap()
}

fn value_into_letter(lv: &LetterValue) -> UpperLetter {
    // can panic if UpperLetter bounds code or next line broken
    UpperLetter::new(u8::from(*lv) + 64).unwrap()
}

type LetterValue = BoundedU8<1, 26>;

type CardValue = BoundedU8<1, 53>;

fn card_val_into_position(cv: &CardValue) -> CardPosition {
    // can panic if value vs position bounds broken (value bounds must be >= position)
    CardPosition::new(u8::from(*cv)).unwrap()
}

fn card_val_into_let_val(cv: CardValue) -> LetterValue {
    // need to convert sum to zero based (-1) before modulo and back to one based (+1) after
    // can panic if the next line broken
    LetterValue::new((u8::from(cv) - 1) % 26 + 1).unwrap()
}

type CardPosition = BoundedU8<1, 54>;

#[derive(Debug, Clone)]
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
            match UpperLetter::new(letter) {
                Some(l) => pt.0.push(l),
                None => continue,
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
            match UpperLetter::new(letter) {
                Some(l) => ct.0.push(l),
                None => continue,
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
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pp = Passphrase(Vec::new());
        for letter in s.to_uppercase().bytes() {
            if let Some(l) = UpperLetter::new(letter) {
                pp.0.push(l);
            } else {
                return Err("string contains non-upper");
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
        // can panic if value table init code broken - not all cards included
        *VALUES.get_or_init(|| {value_init()}).get(self).unwrap()
    }
}

fn value_init() -> HashMap<Card, CardValue> {
    let mut values = HashMap::new();
    // can panics if next line broken - illegal value for JokersPerDeck
    let new_deck = Cards::new(1, JokersPerDeck::new(2).unwrap()); // new deck w/ Joker -> 54 cards
    for (i, card) in new_deck.0.iter().enumerate() {
        if i < 53 {  // have to skip last card as it will generate illegal value
            // can panic if next line broken - illegal card value.
            values.insert(*card, CardValue::new((i + 1) as u8).unwrap());  // values not zero based
        }
    }
    // add the last joker (order is A then B) with same (53) value as the other one
    // can panic if next line broken - illegal card value.
    values.insert(Card::Joker(JokerId::B), CardValue::new(53u8).unwrap());
    values
}

fn main() -> Result<()> {
    init();

    println!();
    trace!("a trace example");
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");

    VALUES.get_or_init(|| {value_init()});

    fn next_deck_state(mut key_deck: Cards) -> Cards {
        // A Joker move
        assert!(key_deck.move_card_circular(Card::Joker(JokerId::A), 0, 1));
        // B Joker move
        assert!(key_deck.move_card_circular(Card::Joker(JokerId::B),0, 2));

        // Triple cut at Jokers (aka fools. fa, fb being fool A and fool B respectively)
        // and swap top with bottom leaving Jokers in place
        // can panic if code broken - required joker not present
        let mut above_fa = key_deck.draw_till(Card::Joker(JokerId::A)).unwrap();
        if let Ok(above_both) = above_fa.draw_till(Card::Joker(JokerId::B)) {
            // Order is: above_both, FB above_fa, FA key_deck
            // can panic if code broken - location of jokers isn't where expected
            let fb = above_fa.draw_count(1).unwrap();
            let fa = key_deck.draw_count(1).unwrap();
            // swap top and bottom leaving fools in same relative positions
            key_deck.append(fb);
            key_deck.append(above_fa);
            key_deck.append(fa);
            key_deck.append(above_both);
        } else if let Ok(mut above_fb) = key_deck.draw_till(Card::Joker(JokerId::B)) {
            // Order is: above_fa, FA above_fb, FB key_deck
            // can panic if code broken - location of jokers isn't where expected
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
            // can panic if code broken - following line doesn't point to location with a card in deck
            look_at(key_deck.0.len() - 1)
            .unwrap()
            .value();
        let TwoStacks(top, mut bottom) = key_deck
            .cut((*bottom_card_value).into());
        let bottom_card = bottom.0
            // can panic if code broken - bottom should have at least the bottom card
            .pop()
            .unwrap();
        bottom.append(top);
        bottom.append(Cards(vec!(bottom_card)));
        bottom.clone()
    }

    fn key_deck_from_passphrase(passphrase: &Passphrase) -> Cards {
        // can panic if code broken - next line uses illegal joker count
        let mut deck = Cards::new(1,JokersPerDeck::new(2).unwrap());

        for letter in passphrase.iter() {
            deck = next_deck_state(deck);

            // count cut at passphrase letter value, maintain bottom card
            let letter_value = letter_into_value(letter);
            let TwoStacks(top, mut bottom) = deck
                .clone()
                .cut(letter_value.into());
            let bottom_card = bottom.0
                // can panic if code broken - bottom should always have a bottom card
                .pop()
                .unwrap();
            bottom.append(top);
            bottom.append(Cards(vec!(bottom_card)));
            deck = bottom.clone();
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
                // can panic if code broken - deck should always have a top card.
                .look_at(0)
                .unwrap()
                .value();
            // hidden canceling adjustments: top_card_value [1..53] so subtract 1 to make it
            // an index range of [0..52] (i.e. so original 1 is pointing to first card)
            // and then add 1 to look at card *after* the one indexed
            // by the top card value for a net adjustment of 0
            let output_card_candidate_position = card_val_into_position(top_card_value);
            let output_card_candidate = &key_deck
            // can panic if code broken - output card should always be present
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

        let mut pt = (*pt).clone();
        while pt.0.len() % 5 != 0 {
            pt.0.push(UpperLetter::new('X' as u8).unwrap());
        }

        let mut ct: CypherText = CypherText(vec!());
        for (i, k) in ks.0.iter().enumerate() {
            let pt_value = letter_into_value(&pt.0[i]);
            // need to convert sum to zero based (-1) before modulo and back to one based (+1) after
            ct.0.push(value_into_letter(
                &(LetterValue::new(
                    ((u8::from(pt_value) + k - 1) % 26) + 1)
                    .unwrap())));
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
            pt.0.push(value_into_letter(
                &(LetterValue::new(
                    (((i16::from(ct_value) - i16::from(*k) - 1)
                        .rem_euclid(26)) + 1) as u8)
                    .unwrap())));
        }
        pt
    }

// --------- TODO - Solitaire Cypher Crate above, below - Move into crate tests  --------------
const ITER_COUNT: usize = 1000;
    let mut metrics = [0usize; ITER_COUNT];
    for i in 0..ITER_COUNT {
        let mut deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
        deck.shuffle_fy();
        metrics[i] = deck.shuffle_rs_metric();
    }
    let sum = metrics.iter().sum::<usize>() as f64;
    let mean = sum / ITER_COUNT as f64;
    println!("the mean metric of {ITER_COUNT} fy randomized new decks is {mean}");


    let mut deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
    println!("New deck: {deck}, shuffle rising sequence metric: {}", deck.shuffle_rs_metric());
    println!("by default values:\n {:?}", deck.by_def_raw_values());
    println!();
    deck.reverse();
    println!("Reversed new deck: {deck}, shuffle rising sequence metric: {}", deck.shuffle_rs_metric());
    println!("by default values:\n {:?}", deck.by_def_raw_values());
    println!();
    let deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
    let TwoStacks(top, bottom) = deck.cut_with_noise(NoiseLevel::new(10).unwrap());
    println!("After cut top: {top}");
    println!();
    println!("After cut bottom: {bottom}");
    println!();
    println!("Cut point: {}", top.0.len());
    println!();
    let mut deck = TwoStacks(top, bottom).merge(MergeType::RANDOM);
    println!("after first riffle:");
    println!("Deck: {deck}, shuffle rising sequence metric: {}", deck.shuffle_rs_metric());
    println!("by default values:\n {:?}", deck.by_def_raw_values());
    println!();
    deck.shuffle(10, NoiseLevel::new(10).unwrap());
    println!("after 10 more riffles:");
    println!("Deck: {deck}, shuffle rising sequence metric: {}", deck.shuffle_rs_metric());
    println!("by default values:\n {:?}", deck.by_def_raw_values());
    println!();
    deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
    deck.shuffle_fy();
    println!("after fully randomizing shuffle");
    println!("Deck: {deck}, shuffle rising sequence metric: {}", deck.shuffle_rs_metric());
    println!("by default values:\n {:?}", deck.by_def_raw_values());
    println!();

    println!("testing cutting of an empty deck");
    let deck = Cards(vec!());
    let TwoStacks(top, bottom) = deck.cut_with_noise(NoiseLevel::new(5).unwrap());
    println!("top: {}, bottom: {}", top, bottom);

    println!("\ntesting 52 perfect In shuffles:");
    let mut deck = Cards::new(1, JokersPerDeck::new(0).unwrap());
    println!("Deck: {deck}, shuffle rising sequence metric: {}", deck.shuffle_rs_metric());
    deck.shuffle(52, NoiseLevel::new(0).unwrap());
    println!("Deck: {deck}, shuffle rising sequence metric: {}", deck.shuffle_rs_metric());

//  -----------TODO - Mover into Solitaire_Cypher testing

    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use regex::Regex;

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn remove_whitespace(s: &str) -> String {
        s.chars().filter(|c|!c.is_whitespace()).collect()
    }

    fn remove_ticks(s: &str) -> String {
        s.chars().filter(|c| *c != '\'').collect()
    }

    fn pad_with_x(s: &str) -> String {
        let mut s = s.to_string();
        while s.len() % 5 != 0 {
            s.push('X');
        }
        s
    }

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





    // println!("First vector test:");
    // let mut new_deck = Cards::new(DeckStyle::Jokers, 1);
    // let pt = PlainText::from_str("AAAAAAAAAAAAAAA").unwrap();
    // let ks = get_key_stream(new_deck, pt.0.len());
    // let ct = encrypt(&pt, &ks);
    // println!("key: <null key>, pt: {}, ks: {}", pt.pt_to_string(), ks.ks_to_string());
    // println!("ct: {:?}", ct.ct_to_string());
    //
    // println!("Second vector test:");
    // let mut new_deck = Cards::new(DeckStyle::Jokers, 1);
    // let passphrase: Passphrase = Passphrase::from_str("f").unwrap();
    // new_deck = key_deck_from_passphrase(&passphrase);
    // let pt = PlainText::from_str("AAAAAAAAAAAAAAA").unwrap();
    // let ks = get_key_stream(new_deck, pt.0.len());
    // let ct = encrypt(&pt, &ks);
    // println!("key: {}, pt: {}, ks: {}", passphrase.pp_to_string(), pt.pt_to_string(), ks.ks_to_string());
    // println!("ct: {:?}", ct.ct_to_string());
    //
    // println!("third vector test:");
    // let mut new_deck = Cards::new(DeckStyle::Jokers, 1);
    // let passphrase: Passphrase = Passphrase::from_str("fo").unwrap();
    // new_deck = key_deck_from_passphrase(&passphrase);
    // let pt = PlainText::from_str("AAAAAAAAAAAAAAA").unwrap();
    // let ks = get_key_stream(new_deck, pt.0.len());
    // let ct = encrypt(&pt, &ks);
    // println!("key: {}, pt: {}, ks: {}", passphrase.pp_to_string(), pt.pt_to_string(), ks.ks_to_string());
    // println!("ct: {:?}", ct.ct_to_string());
    //
    // println!("eleventh vector test:");
    // let mut new_deck = Cards::new(DeckStyle::Jokers, 1);
    // let passphrase: Passphrase = Passphrase::from_str("cryptonomicon").unwrap();
    // new_deck = key_deck_from_passphrase(&passphrase);
    // let pt = PlainText::from_str("AAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
    // let ks = get_key_stream(new_deck, pt.0.len());
    // let ct = encrypt(&pt, &ks);
    // println!("key: {}, pt: {}, ks: {}", passphrase.pp_to_string(), pt.pt_to_string(), ks.ks_to_string());
    // println!("ct: {:?}", ct.ct_to_string());
    //
    // println!("twelveth vector test:");
    // let mut new_deck = Cards::new(DeckStyle::Jokers, 1);
    // let passphrase: Passphrase = Passphrase::from_str("cryptonomicon").unwrap();
    // new_deck = key_deck_from_passphrase(&passphrase);
    // let mut spare_deck = new_deck.clone();
    // let pt = PlainText::from_str("SOLITAIRE").unwrap();
    // let ks = get_key_stream(new_deck, pt.0.len());
    // let ct = encrypt(&pt, &ks);
    // println!("key: {}, pt: {}, ks: {}", passphrase.pp_to_string(), pt.pt_to_string(), ks.ks_to_string());
    // println!("ct: {:?}", ct.ct_to_string());
    //
    // let ks = get_key_stream(spare_deck, pt.0.len());
    // let pt = decrypt(&ct, &ks);
    // println!("pt: {:?}", pt.pt_to_string());


    // Open the file of test vectors "sol-test.txt" and run all the enclosed tests
    // Example (repeating) file format:

    // Plaintext:  AAAAAAAAAAAAAAA
    // Key: <null key>
    // Output: 4 49 10 53 24 8 51 44 6 4 33 20 39 19 34 42
    // Ciphertext: EXKYI ZSGEH UNTIQ
    //

    // which repeats for each test.



    let pt_re = Regex::new(r"^Plaintext: +([A-Z]+) *$").unwrap();
    let ct_re = Regex::new(r"^Ciphertext: +((([A-Z]{5}+) *)+) *$").unwrap();
    let key_re = Regex::new(r"^Key: +('([a-z]+)'|(<null key>)) *$").unwrap();
    if let Ok(lines) = read_lines("./sol-test.txt") {
        println!();

        let mut pt: PlainText = PlainText::new();
        let mut pp: Passphrase = Passphrase::from_str("").unwrap();
        let mut ct: CypherText;
        let mut key_deck: Cards;
        let mut ks: KeyStream;

        for line in lines.flatten() {
            if let Some(pt_str) = pt_re.captures(&line) {
                println!("pt: {:?}", &pt_str[1]);
                pt = PlainText::from_str(&pt_str[1]).unwrap();
            } else if let Some(pp_str) = key_re.captures(&line) {
                 println!("pf: {:?}", remove_ticks(&pp_str[1]));
                if let Ok(ppp) = Passphrase::from_str(&remove_ticks(&pp_str[1])) {
                    pp = ppp;
                } else {
                    pp = Passphrase::from_str("").unwrap();
                }
            } else if let Some(ct_str) = ct_re.captures(&line) {
                println!("ct: {:?}", remove_whitespace(&ct_str[1]));
                ct = CypherText::from_str(&remove_whitespace(&ct_str[1])).unwrap();

                key_deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
                if pp.0.len() > 0 {
                    key_deck = key_deck_from_passphrase(&pp);
                }
                ks = get_key_stream(key_deck, pt.0.len());
                let computed_ct = encrypt(&pt, &ks);
                let computed_ct_str = computed_ct.ct_to_string();
                let ct_str = ct.ct_to_string();
                assert_eq!(computed_ct_str, ct_str);
                let computed_pt = decrypt(&ct, &ks);
                let computed_pt_str = computed_pt.pt_to_string();
                let pt_str = pad_with_x(&pt.pt_to_string());
                assert_eq!(computed_pt_str, pt_str);
                println!("PASS\n");
            }
        }
    } else {
        println!("Failed to open test file");
    }

    Ok(())
}
