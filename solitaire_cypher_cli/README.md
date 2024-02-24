## solitaire_cypher_cli

 An implementation of the playing card based cypher created by
 Bruce Schneier and featured in Neal Stephenson’s "Cryptonomicon".
 Encrypts or Decrypts stdin to stdout based on the command line provided passphrase.
 Returns error if the passphrase includes any non-letter characters.
 Crate solitaire_cypher exists to provide these, and more, functions in a lib.
 See: <https://www.schneier.com/academic/solitaire/> and, of course, read Cryptonomicon!

## Installation

cargo install solitaire_cypher_cli

## Examples
 ```
$ solitaire_cypher --help
 Usage: solitaire_cypher --passphrase <PASSPHRASE> <--encrypt|--decrypt>

 Options:
 -e, --encrypt                  Encrypt stdin with keystream generated from passphrase
 -d, --decrypt                  Decrypt stdin with keystream generated from passphrase
 -p, --passphrase <PASSPHRASE>  passphrase (letters only) for key generation
 -h, --help                     Print help
 -V, --version                  Print version
 $ echo "SOLITAIRE" | ./solitaire_cypher --passphrase cryptonomicon --encrypt
 KIRAK SFJAN
 $ echo "KIRAK SFJAN" | ./solitaire_cypher --passphrase cryptonomicon --decrypt
 SOLITAIREX
 $
 ```

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
