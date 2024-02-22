## Solitaire

Rust workspace for three rust crates (all available on crates.io and documentated on docs.rs):
- solitaire_cypher_cli: command line interface for encrypting and decrypting
- solitaire_cypher: lib with more capability than the cli tool
- card_play: lib with tools for manipulating decks of cards

## solitaire_cypher_cli

An implementation of the playing card based cypher created by
Bruce Schneier and featured in Neal Stephenson’s "Cryptonomicon".
Encrypts or Decrypts stdin to stdout based on the command line provided passphrase.
Returns error if the passphrase includes any non-letter characters.
Crate solitaire_cypher exists to provide these, and more, functions in a lib.
See: <https://www.schneier.com/academic/solitaire/> and, of course, read Cryptonomicon!

## solitaire_cypher

An implementation of the playing card based cypher created by
Bruce Schneier and featured in Neal Stephenson’s "Cryptonomicon".
Includes crate solitaire_cypher_cli for command line usage.
See: <https://www.schneier.com/academic/solitaire/> and, of course, read Cryptonomicon!

## card_play

A set of types, methods and functions for manipulating playing cards (common french-suited with or
without jokers). Support for cutting, merging, both human style and fully random ordering shuffles,
measuring shuffle quality (rising sequence based), drawing cards, moving cards in a deck
etc.  Handles card stacks containing more than one deck.
Target user is someone who want to manipulate the deck such as magicians, etc.
rather than users looking for an engine for card games.  The solitaire_cypher was the first
use.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

