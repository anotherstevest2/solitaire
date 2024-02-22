#![warn(missing_docs)]
//! # Solitaire Cypher Cli
//!
//! An implementation of the playing card based cypher created by
//! Bruce Schneier and featured in Neal Stephenson’s "Cryptonomicon".
//! Encrypts or Decrypts stdin to stdout based on the command line provided passphrase.
//! Returns error if the passphrase includes any non-letter characters.
//! Crate solitaire_cypher exists to provide these, and more, functions in a lib.
//! See: <https://www.schneier.com/academic/solitaire/> and, of course, read Cryptonomicon!
//!
//! #Examples
//! ```
//!$ ./solitaire_cypher --help
//! Usage: solitaire_cypher --passphrase <PASSPHRASE> <--encrypt|--decrypt>
//!
//! Options:
//! -e, --encrypt                  Encrypt stdin with keystream generated from passphrase
//! -d, --decrypt                  Decrypt stdin with keystream generated from passphrase
//! -p, --passphrase <PASSPHRASE>  passphrase (letters only) for key generation
//! -h, --help                     Print help
//! -V, --version                  Print version
//! $ echo "SOLITAIRE" | ./solitaire_cypher --passphrase cryptonomicon --encrypt
//! KIRAK SFJAN
//! $ echo "KIRAK SFJAN" | ./solitaire_cypher --passphrase cryptonomicon --decrypt
//! SOLITAIREX
//! $
//! ```

use anyhow::Result;
use clap::{Args, Parser};
use solitaire_cypher::*;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::{fmt, io};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Mutually exclusive command flags
    #[command(flatten)]
    cmd: Cmd,

    /// passphrase for (letters only) key generation
    #[arg(short, long)]
    passphrase: String,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Cmd {
    /// Encrypt stdin with keystream generated from passphrase
    #[arg(short, long)] //
    encrypt: bool,
    /// Decrypt stdin with keystream generated from passphrase
    #[arg(short, long)]
    decrypt: bool,
}

#[derive(Debug)]
struct IllegalArgumentFormatError;
impl Display for IllegalArgumentFormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "IllegalArgumentFormatError - only letters allowed")
    }
}
impl std::error::Error for IllegalArgumentFormatError {}

#[derive(Debug)]
struct IllegalInputFormatError;
impl Display for IllegalInputFormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "IllegalInputFormatError - only letters allowed")
    }
}
impl std::error::Error for IllegalInputFormatError {}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let (enc, dec) = ((&cli.cmd).encrypt, (&cli.cmd).decrypt);

    let encrypting = match (enc, dec) {
        (true, _) => true,
        (_, true) => false,
        _ => unreachable!(),
    };

    let mut stdin = io::read_to_string(io::stdin())?;
    remove_whitespace(&mut stdin);

    let passphrase = match Passphrase::from_str(&cli.passphrase) {
        Ok(passphrase) => passphrase,
        Err(e) => {
            eprintln!("{}", e);
            return Err(IllegalArgumentFormatError.into());
        }
    };

    let key_deck = key_deck_from_passphrase(&passphrase);

    let output = if encrypting {
        let pt = match PlainText::from_str(&stdin) {
            Ok(pt) => pt,
            Err(e) => {
                eprintln!("{}", e);
                return Err(IllegalInputFormatError.into());
            }
        };
        let ks = get_key_stream(key_deck, pt.len());
        encrypt(&pt, &ks).to_string()
    } else {
        let ct = match CypherText::from_str(&stdin) {
            Ok(ct) => ct,
            Err(e) => {
                eprintln!("{}", e);
                return Err(IllegalInputFormatError.into());
            }
        };
        let ks = get_key_stream(key_deck, ct.len());
        decrypt(&ct, &ks).to_string()
    };

    println!("{}", output);
    Ok(())
}
