# `scream_cypher`

[![Crates.io Version](https://img.shields.io/crates/v/scream_cypher)](https://crates.io/crates/scream_cypher/versions)
[![Crates.io Size](https://img.shields.io/crates/size/scream_cypher)](https://crates.io/crates/scream_cypher)
[![docs.rs](https://img.shields.io/docsrs/scream_cypher)](https://docs.rs/scream_cypher)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/icorbrey)](https://github.com/sponsors/icorbrey)

In the **SCREAM CYPHER**, messages consist of all As with different letters
distinguished using diacritics. This is a tool that provides both a CLI tool
and a library to encrypt and decrypt text using the scream cypher.

## Acknowledgements

This cypher originated from [XKCD](https://xkcd.com/3054/). Thank you, Randall
Munroe, for always bringing such beautiful things into this world.

<img src="https://imgs.xkcd.com/comics/scream_cipher_2x.png" width=325>

## Command line installation and usage

Install `scream_cypher` with Cargo:

```sh
cargo install scream_cypher
```

You can then use the `scream` command to encrypt and decrypt messages:

```sh
scream encrypt "This is a test."
# Āa̰ảã ảã a āáãā.

scream decrypt "Āa̰ảã ảã a āáãā."
# This is a test.
```

## Library installation and usage

Add `scream_cypher` to your project:

```sh
cargo add scream_cypher
```

You can then use `scream_cypher::encrypt` and `scream_cypher::encrypt` to
encrypt and decrypt messages:

```rs
let ciphertext = scream_cipher::encrypt("This is a test.");

println!("Your message: \"{}\"", ciphertext);
// Your message: "Āa̰ảã ảã a āáãā."

let plaintext = scream_cipher::decrypt(cyphertext);

println!("Your message: \"{}\"", plaintext);
// Your message: "This is a test."
```

## License

This project is dual licensed under [Apache 2.0](./LICENSE-APACHE) or
[MIT](./LICENSE-MIT).
