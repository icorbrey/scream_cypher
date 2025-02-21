use clap::{arg, command, Command};

const ENCRYPT: &str = "encrypt";
const DECRYPT: &str = "decrypt";

fn main() {
    let matches = command!()
        .subcommand(
            Command::new(ENCRYPT)
                .about("Encrypts a plaintext message using the scream cypher.")
                .arg(arg!(<message> "The message to encrypt.")),
        )
        .subcommand(
            Command::new(DECRYPT)
                .about("Decrypts an encrypted message using the scream cypher.")
                .arg(arg!(<message> "The message to decrypt")),
        )
        .arg_required_else_help(true)
        .get_matches();

    match matches.subcommand() {
        Some((ENCRYPT, sub_matches)) => {
            println!(
                "{}",
                sub_matches
                    .get_one::<String>("message")
                    .map(|message| scream_cipher::encrypt(message))
                    .expect("<message> should have been populated")
            )
        }
        Some((DECRYPT, sub_matches)) => {
            println!(
                "{}",
                sub_matches
                    .get_one::<String>("message")
                    .map(|message| scream_cipher::decrypt(message))
                    .expect("<message> should have been populated")
            )
        }
        _ => panic!(),
    }
}
