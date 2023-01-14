# playfair-rs

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/orhun/playfair-rs/ci.yml)](https://github.com/orhun/playfair-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/playfair_rs)](https://crates.io/crates/playfair_rs)
[![docs.rs](https://img.shields.io/docsrs/playfair-rs)](https://docs.rs/playfair_rs/latest)
[![Codecov](https://img.shields.io/codecov/c/gh/orhun/playfair-rs)](https://app.codecov.io/gh/orhun/playfair-rs)

[Playfair cipher](https://en.wikipedia.org/wiki/Playfair_cipher) implemented in Rust.

## Usage

```rs
fn main() {
  let encrypted = playfair_rs::encrypt("playfair example", "hide the gold in the tree stump", 'x').unwrap();
  println!("{encrypted}"); // bmodzbxdnabekudmuixmmouvif

  let decrypted = playfair_rs::decrypt("playfair example", &encrypted).unwrap();
  println!("{decrypted}"); // hidethegoldinthetrexestump
}
```

## License

All code is dual-licensed under [The MIT License](./LICENSE-MIT) and [Apache 2.0 License](./LICENSE-APACHE).

## Copyright

Copyright © 2023, [Orhun Parmaksız](mailto:orhunparmaksiz@gmail.com)
