use std::str::FromStr;
use card_play::*;
use sdk::*;

use solitaire_cypher::*;


fn main() -> Result<()> {
    sdk_init();

//     println!();
//     trace!("a trace example");
//     debug!("deboogging");
//     info!("such information");
//     warn!("o_O");
//     error!("boom");
//
//     // Manual tests of card_play
// const ITER_COUNT: usize = 1000;
//     let mut metrics = [0usize; ITER_COUNT];
//     for i in 0..ITER_COUNT {
//         let mut deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
//         deck.shuffle_fy();
//         metrics[i] = deck.shuffle_rs_metric();
//     }
//     let sum = metrics.iter().sum::<usize>() as f64;
//     let mean = sum / ITER_COUNT as f64;
//     println!("the mean metric of {ITER_COUNT} fy randomized new decks is {mean}");
//
//
//     let mut deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
//     println!("New deck: {deck}, shuffle rising sequence metric: {}", deck.shuffle_rs_metric());
//     println!("by default values:\n {:?}", deck.by_def_raw_values());
//     println!();
//     deck.reverse();
//     println!("Reversed new deck: {deck}, shuffle rising sequence metric: {}", deck.shuffle_rs_metric());
//     println!("by default values:\n {:?}", deck.by_def_raw_values());
//     println!();
//     let deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
//     let TwoStacks(top, bottom) = deck.cut_with_noise(NoiseLevel::new(10).unwrap());
//     println!("After cut top: {top}");
//     println!();
//     println!("After cut bottom: {bottom}");
//     println!();
//     println!("Cut point: {}", top.0.len());
//     println!();
//     let mut deck = TwoStacks(top, bottom).merge(MergeType::RANDOM);
//     println!("after first riffle:");
//     println!("Deck: {deck}, shuffle rising sequence metric: {}", deck.shuffle_rs_metric());
//     println!("by default values:\n {:?}", deck.by_def_raw_values());
//     println!();
//     deck.shuffle(10, NoiseLevel::new(10).unwrap());
//     println!("after 10 more riffles:");
//     println!("Deck: {deck}, shuffle rising sequence metric: {}", deck.shuffle_rs_metric());
//     println!("by default values:\n {:?}", deck.by_def_raw_values());
//     println!();
//     deck = Cards::new(1, JokersPerDeck::new(2).unwrap());
//     deck.shuffle_fy();
//     println!("after fully randomizing shuffle");
//     println!("Deck: {deck}, shuffle rising sequence metric: {}", deck.shuffle_rs_metric());
//     println!("by default values:\n {:?}", deck.by_def_raw_values());
//     println!();
//
//     println!("testing cutting of an empty deck");
//     let deck = Cards(vec!());
//     let TwoStacks(top, bottom) = deck.cut_with_noise(NoiseLevel::new(5).unwrap());
//     println!("top: {}, bottom: {}", top, bottom);
//
//     println!("\ntesting 52 perfect In shuffles:");
//     let mut deck = Cards::new(1, JokersPerDeck::new(0).unwrap());
//     println!("Deck: {deck}, shuffle rising sequence metric: {}", deck.shuffle_rs_metric());
//     deck.shuffle(52, NoiseLevel::new(0).unwrap());
//     println!("Deck: {deck}, shuffle rising sequence metric: {}", deck.shuffle_rs_metric());

//  -----------TODO - Mover into Solitaire_Cypher testing

    // Manual tests of solitaire_cypher
    // ----------- TODO - Move the internal cypher operation tests into the solitaire_cypher crate
    // ----------- TODO - Reconfigure use etc. so that most of card_play is not exposed and wrap
    // -----------        What make sense for users of the Cypher code for availability via the
    // -----------        cypher crate

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





    // println!("\ntwelveth vector test:");
    // let passphrase: Passphrase = Passphrase::from_str("cryptonomicon").unwrap();
    // let new_deck = key_deck_from_passphrase(&passphrase);
    // let spare_deck = new_deck.clone();
    // let pt = PlainText::from_str("SOLITAIRE").unwrap();
    // let ks = get_key_stream(new_deck, pt.0.len());
    // let ct = encrypt(&pt, &ks);
    // println!("key: {}, pt: {}, ks: {}", passphrase.to_string(), pt.to_string(), ks.to_string());
    // println!("ct: {:?}", ct.to_string());
    //
    // let ks = get_key_stream(spare_deck, pt.0.len());
    // let pt = decrypt(&ct, &ks);
    // println!("pt: {:?}", pt.to_string());
    //
    // println!("\nFrom Book:");
    // let passphrase: Passphrase = Passphrase::from_str("cryptonomicon").unwrap();
    // let new_deck = key_deck_from_passphrase(&passphrase);
    // let spare_deck = new_deck.clone();
    // let pt = PlainText::from_str("SOLITAIRE").unwrap();
    // let ks = get_key_stream(new_deck, pt.0.len());
    // let ct = encrypt(&pt, &ks);
    // println!("key: {}, pt: {}, ks: {}", passphrase.to_string(), pt.to_string(), ks.to_string());
    // println!("ct: {:?}", ct.to_string());
    //
    // let ks = get_key_stream(spare_deck, pt.0.len());
    // let pt = decrypt(&ct, &ks);
    // println!("pt: {:?}", pt.to_string());





    Ok(())
}
