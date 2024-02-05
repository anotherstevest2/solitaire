// TODO - enhance to/from string for cypher and plain text to remove white space when not string
//        add a space every five chars when going to string.

// TODO - When refactoring - consider doing newtype pattern on Bounded variables so that from can be
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

pub type UpperLetter = BoundedU8<65, 90>;

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
pub struct PlainText (pub Vec<UpperLetter>);

impl PlainText {
    pub fn new() -> PlainText {
        PlainText(Vec::new())
    }

    /// Converts a PlainText into a String
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for c in self.0.iter() {
            s.push(u8::from(*c) as char);
        }
        s
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for PlainText {
    type Err = Infallible;

    /// Creates a PlainText from a slice of letters - lower case letters are mapped to upper during
    /// creation.  Non-letters are ignored (including spaces) and
    /// 'X' is used to pad the end so length is a multiple of five (as is the
    /// crypto tradition).
    ///
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use solitaire_cypher::PlainText;
    /// let ct_result = PlainText::from_str("Don't use PC");
    /// assert!(ct_result.is_ok());
    /// println!("CypherText: {}", ct_result.unwrap().to_string());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pt = PlainText(Vec::new());
        for letter in s.to_uppercase().bytes() {
            match UpperLetter::new(letter) {
                Some(l) => pt.0.push(l),
                None => continue,
            }
        }
        while pt.0.len() % 5 != 0 {
            pt.0.push(UpperLetter::new('X' as u8).unwrap());
        }
        Ok(pt)
    }
}

#[derive(Debug)]
pub struct CypherText (pub Vec<UpperLetter>);
impl CypherText {
    pub fn new() -> CypherText {
        CypherText(Vec::new())
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for (i, c) in self.0.iter().enumerate() {
            if i != 0 && i % 5 == 0 {
                s.push(' ');
            }
            s.push(u8::from(*c) as char);
        }
        s
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for CypherText {
    type Err = Infallible;

    /// Creates a CypherText from a slice of letters - lower case letters are mapped to upper during
    /// creation.
    ///
    ///
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use solitaire_cypher::CypherText;
    /// let ct_result = CypherText::from_str("GibberishLetters");
    /// assert!(ct_result.is_ok());
    /// println!("CypherText: {}", ct_result.unwrap().to_string());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ct = CypherText(Vec::new());
        for letter in s.to_uppercase().bytes() {
            match UpperLetter::new(letter) {
                Some(l) => ct.0.push(l),
                None => continue,
            }
        }
        Ok(ct)
    }
}

#[derive(Debug)]
pub struct Passphrase (pub Vec<UpperLetter>);

impl Passphrase {
    fn iter(&self) -> std::slice::Iter<'_, UpperLetter> {
        self.0.iter()
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for c in self.0.iter() {
            s.push(u8::from(*c) as char);
        }
        s
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for Passphrase {
    type Err = &'static str;

/// Creates a Passphrase from a slice of letters - all lower case letters mapped to upper during
/// creation.  Single quotes are removed to ease use of wikipedia's solitaire cypher test vectors
///
/// returns Err string if non-letters are encountered in the slice
///
/// # Examples
/// ```
/// use std::str::FromStr;
/// use solitaire_cypher::Passphrase;
/// let ps_result = Passphrase::from_str("cryptoNOMicon");
/// assert!(ps_result.is_ok());
/// println!("Passphrase: {}",ps_result.unwrap().to_string());
/// assert!(Passphrase::from_str("crypto NOMicon").is_err());
/// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pp = Passphrase(Vec::new());
        let s = &remove_ticks(s);
        for letter in s.to_uppercase().bytes() {
            if let Some(l) = UpperLetter::new(letter) {
                pp.0.push(l);
            } else {
                return Err("string contains non-letter");
            }
        }
        Ok(pp)
    }
}

pub struct KeyStream (pub Vec<UpperLetter>);

impl KeyStream {
    pub fn new() -> KeyStream {
        KeyStream(Vec::new())
    }
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for (i, c) in self.0.iter().enumerate() {
            if i != 0 && i % 5 == 0 {
                s.push(' ');
            }
            s.push(u8::from(*c) as char);
        }
        s
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for KeyStream {
    type Err = Infallible;

    /// Creates a KeyStream from a slice of letters - lower case letters are mapped to upper during
    /// creation.
    ///
    ///
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use solitaire_cypher::KeyStream;
    /// let ks_result = KeyStream::from_str("GibberishLetters");
    /// assert!(ks_result.is_ok());
    /// assert_eq!("GIBBE RISHL ETTER S".to_string(), ks_result.unwrap().to_string());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ks = KeyStream(Vec::new());
        for letter in s.to_uppercase().bytes() {
            match UpperLetter::new(letter) {
                Some(l) => ks.0.push(l),
                None => continue,
            }
        }
        Ok(ks)
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

/// Create a key deck from a Passphrase (aka key)
///
/// # Examples
/// ```
/// use std::str::FromStr;
/// use solitaire_cypher::{key_deck_from_passphrase, Passphrase};
/// let passphrase = Passphrase::from_str("cryptonomicon").unwrap();
/// let keyed_deck = key_deck_from_passphrase(&passphrase);
/// ```
///
pub fn key_deck_from_passphrase(passphrase: &Passphrase) -> Cards {
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

/// Create a KeyStream of the specified length from a Card deck
///
/// Examples
/// ```
/// use card_play::{Cards, JokersPerDeck};
/// use solitaire_cypher::get_key_stream;
/// let deck = Cards::new(1, JokersPerDeck::new(2).unwrap()); // un-keyed deck example
/// let len = 10; // should be at least as long as the text being encrypted/decrypted
/// let ks = get_key_stream(deck, len);
/// ```
pub fn get_key_stream(key_deck: Cards, key_length: usize) -> KeyStream {
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
                .push(value_into_letter(&card_val_into_let_val((*output_card_candidate).value())));
        }
    }
    key_stream
}

/// Encrypt PlainText into CypherText using the given KeyStream
///
/// Examples
/// ```
/// use std::str::FromStr;
/// use solitaire_cypher::{encrypt, KeyStream, Passphrase, PlainText};
/// let pt = PlainText::from_str("DONOT USEPC").unwrap();
/// let ks = KeyStream::from_str("KDWUP ONOWT").unwrap();
/// let ct = encrypt(&pt, &ks);
/// assert_eq!(ct.to_string(), "OSKJJ JGTMW");
/// ```
pub fn encrypt(pt: &PlainText, ks: &KeyStream) -> CypherText {
    if pt.0.len() > ks.0.len() {
        panic!("KeyStream not long enough");
    }

    let pt = (*pt).clone();
    let mut ct: CypherText = CypherText(vec!());
    for (i, k) in ks.0.iter().enumerate() {
        let pt_value = letter_into_value(&pt.0[i]);
        // need to convert sum to zero based (-1) before modulo and back to one based (+1) after
        ct.0.push(value_into_letter(
            &(LetterValue::new(
                ((u8::from(pt_value) + letter_into_value(k) - 1) % 26) + 1)
                .unwrap())));
    }
    ct
}

/// Decrypt CypherText into PlainText using the given KeyStream
///
/// Examples
/// ```
/// use std::str::FromStr;
/// use solitaire_cypher::{CypherText, decrypt, encrypt, get_key_stream, key_deck_from_passphrase, Passphrase, PlainText};
/// let ct = CypherText::from_str("KIRAK SFJAN").unwrap();
/// let passphrase: Passphrase = Passphrase::from_str("cryptonomicon").unwrap();
/// let mut key_deck = key_deck_from_passphrase(&passphrase);
/// let ks = get_key_stream(key_deck, ct.len());
/// let recovered_pt = decrypt(&ct, &ks);
/// assert_eq!("SOLITAIREX", recovered_pt.to_string());
/// ```
pub fn decrypt(ct: &CypherText, ks: &KeyStream) -> PlainText {

    if ct.0.len() > ks.0.len() {
        panic!("KeyStream not long enough");
    }

    let mut pt: PlainText = PlainText(vec!());
    for (i, k) in ks.0.iter().enumerate() {
        let ct_value = letter_into_value(&ct.0[i]);
        pt.0.push(value_into_letter(
            &(LetterValue::new(
                (((i16::from(ct_value) - i16::from(letter_into_value(k)) - 1)
                    .rem_euclid(26)) + 1) as u8)
                .unwrap())));
    }
    pt
}

fn remove_ticks(s: &str) -> String {
        s.chars().filter(|c| *c != '\'').collect()
    }

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use regex::Regex;
    use super::*;

    # [test]
    // Open the file of test vectors "sol-test.txt" and run all the enclosed tests
    // Example (repeating) file format:

    //     Plaintext:  AAAAAAAAAAAAAAA
    //     Key:  'bcd'
    //     Output:  5 38 20 27 50 1 38 26 49 33 39 42 49 2 35
    //     Ciphertext:  FMUBY BMAXH NQXCJ
    //
    // which repeats for each test.
    fn test_vectors_from_wiki() {
        let pt_re = Regex::new(r"^Plaintext: +([A-Z]+) *$").unwrap();
        let ct_re = Regex::new(r"^Ciphertext: +((([A-Z]{5}+) *)+) *$").unwrap();
        let key_re = Regex::new(r"^Key: +('([a-z]+)'|(<null key>)) *$").unwrap();
        if let Ok(lines) = read_lines("./sol-test.txt") {
            let mut pt: PlainText = PlainText::new();
            let mut pp: Passphrase = Passphrase::from_str("").unwrap();
            let mut ct: CypherText;
            let mut key_deck: Cards;
            let mut ks: KeyStream;

            for line in lines.flatten() {
                if let Some(pt_str) = pt_re.captures(&line) {
                    pt = PlainText::from_str(&pt_str[1]).unwrap();
                } else if let Some(pp_str) = key_re.captures(&line) {
                    if let Ok(ppp) = Passphrase::from_str(&pp_str[1]) {
                        pp = ppp;
                    } else {
                        pp = Passphrase::from_str("").unwrap();
                    }
                } else if let Some(ct_str) = ct_re.captures(&line) {
                    ct = CypherText::from_str(&ct_str[1]).unwrap();

                    key_deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
                    if pp.0.len() > 0 {
                        key_deck = key_deck_from_passphrase(&pp);
                    }
                    ks = get_key_stream(key_deck, pt.0.len());
                    let computed_ct = encrypt(&pt, &ks);
                    let computed_ct_str = computed_ct.to_string();
                    let ct_str = ct.to_string();
                    assert_eq!(computed_ct_str, ct_str);
                    let computed_pt = decrypt(&ct, &ks);
                    let computed_pt_str = computed_pt.to_string();
                    assert_eq!(computed_pt_str, pt.to_string());
                }
            }
        } else {
            println!("Failed to open test file");
        }

    }
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}