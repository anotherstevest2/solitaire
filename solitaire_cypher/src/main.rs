use std::str::FromStr;
use card_play::*;
use sdk::*;

use solitaire_cypher::*;


fn main() -> Result<()> {
    sdk_init();
    VALUES.get_or_init(|| {value_init()});

    println!();
    trace!("a trace example");
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");

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
