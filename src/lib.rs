//! # `scream_cypher`
//!
//! In the **SCREAM CYPHER**, messages consist of all As with different letters
//! distinguished using diacritics. This is a tool that provides both a CLI
//! tool and a library to encrypt and decrypt text using the scream cypher.
//!
//! ## Acknowledgements
//!
//! This cypher originated from [XKCD](https://xkcd.com/3054/). Thank you,
//! Randall Munroe, for always bringing such beautiful things into this world.
//!
//! ## Command line installation and usage
//!
//! Install `scream_cypher` with Cargo:
//!
//! ```sh
//! cargo install scream_cypher
//! ```
//!
//! You can then use the `scream` command to encrypt and decrypt messages:
//!
//! ```sh
//! scream encrypt "This is a test."
//! # Āa̰ảã ảã a āáãā.
//!
//! scream decrypt "Āa̰ảã ảã a āáãā."
//! # This is a test.
//! ```
//!
//! ## Library installation and usage
//!
//! Add `scream_cypher` to your project:
//!
//! ```sh
//! cargo add scream_cypher
//! ```
//!
//! You can then use `scream_cypher::encrypt` and `scream_cypher::encrypt` to
//! encrypt and decrypt messages:
//!
//! ```rs
//! let ciphertext = scream_cypher::encrypt("This is a test.");
//!
//! println!("Your message: \"{}\"", ciphertext);
//! // Your message: "Āa̰ảã ảã a āáãā."
//!
//! let plaintext = scream_cypher::decrypt(cyphertext);
//!
//! println!("Your message: \"{}\"", plaintext);
//! // Your message: "This is a test."
//! ```

use phf::{phf_map, Map};

/// A map of alpha characters to their corresponding scream cypher encoding.
///
/// See: <https://xkcd.com/3054/>
pub const CYPHER_MAP: Map<&'static str, &'static str> = phf_map! {
    "A" => "A",
    "B" => "A\u{0307}",
    "C" => "A\u{0321}",
    "D" => "A\u{0331}",
    "E" => "A\u{0301}",
    "F" => "A\u{032E}",
    "G" => "A\u{030B}",
    "H" => "A\u{0330}",
    "I" => "A\u{0309}",
    "J" => "A\u{0313}",
    "K" => "A\u{0323}",
    "L" => "A\u{0306}",
    "M" => "A\u{030C}",
    "N" => "A\u{0302}",
    "O" => "A\u{030A}",
    "P" => "A\u{032F}",
    "Q" => "A\u{0324}",
    "R" => "A\u{0311}",
    "S" => "A\u{0303}",
    "T" => "A\u{0304}",
    "U" => "A\u{0308}",
    "V" => "A\u{0300}",
    "W" => "A\u{030F}",
    "X" => "A\u{036F}",
    "Y" => "A\u{0326}",
    "Z" => "A\u{0337}",
};

/// Encrypts the given plaintext using the scream cypher.
///
/// See: <https://xkcd.com/3054/>
///
/// ## Example
///
/// ```
/// let ciphertext = scream_cypher::encrypt("This is a test.");
///
/// println!("Your message: \"{}\"", ciphertext);
/// /// Your message: "Āa̰ảã ảã a āáãā."
/// # assert_eq!("Āa̰ảã ảã a āáãā.", ciphertext);
/// ```
pub fn encrypt(message: &str) -> String {
    let mut result = message.to_owned();

    for (plain, cypher) in CYPHER_MAP.entries() {
        result = result
            .replace(plain, cypher)
            .replace(&plain.to_lowercase(), &cypher.to_lowercase());
    }

    result
}

/// Decrypts the given cyphertext using the scream cypher.
///
/// See: <https://xkcd.com/3054/>
///
/// ## Example
///
/// ```
/// let plaintext = scream_cypher::decrypt("Āa̰ảã ảã a āáãā.");
///
/// println!("Your message: \"{}\"", plaintext);
/// /// Your message: "This is a test."
/// # assert_eq!("This is a test.", plaintext);
/// ```
pub fn decrypt(message: &str) -> String {
    let mut result = message.to_owned();

    for (plain, cypher) in CYPHER_MAP.entries() {
        result = result
            .replace(cypher, plain)
            .replace(&cypher.to_lowercase(), &plain.to_lowercase());
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encrypted_messages_decrypt_to_original_messages() {
        let messages = [
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            "abcdefghijklmnopqrstuvwxyz",
            "This is a test of the emergency broadcast system.",
        ];

        for &message in messages.iter() {
            assert_eq!(message, decrypt(&encrypt(message)));
        }
    }
}
