# Engish
<div id="top"></div>

[![Latest Version]][crates.io] 
[![docs]][docs.rs]
[![Minimum Supported Rust Version]][Rust]

A strongly opinionated crate that supports messing around with language (in ways you probably shouldn't). 

<!-- ABOUT THE PROJECT -->
## About The Project

This project provides functionality for procedurally building pseudo words, as well as basic dictionary sampling.
Nouns, Verbs and Adjectives are defined and constructable, with grammatical rules available for changing tense, pluralizing etc.
Due to the nature of "rules" and the English language, you should not use this for anything that falls outside the remit of "silly". 

The only data that comes with this crate is for letter frequencies- if you want to build a dictionary then you need to provide your own data sets. 



<!-- GETTING STARTED -->
## Getting Started

As with most rust crates, this can be imported to a project using [crates.io](https://crates.io/crates). Follow the link for more infomation.

## Usage

First, add `engish` to your `Cargo.toml`. To use the word builders, you'll need to enable the `builders` feature.

```toml
[dependencies]
engish = { version = "0.3", features = ["builders"] }
```
*(Note: check [crates.io] for the latest version number.)*

Here is an example of how to generate a pseudo-word using the `PropperNounBuilder`:

```rust
use engish::prelude::*;

fn main() {
    // Build a new 'propper' noun with a length between 5 and 10 characters.
    let noun = PropperNounBuilder::new()
        .build();

    println!("Generated proper noun: {}", noun);
}
```

### Optional features
* [`builders`] - adds functionality for word building.


## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT)
  or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Contributions are both welcome and appreciated!

Contributions in any form (issues, pull requests, etc.) to this project must
adhere to Rust's [Code of Conduct].

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

<!-- CONTACT -->
## Contact

Caspar Green - caspar.m.green@gmail.com

Project Link: [https://github.com/fishykins/engish](https://github.com/fishykins/engish)

<p align="right">(<a href="#top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[Latest Version]: https://img.shields.io/crates/v/engish.svg
[crates.io]: https://crates.io/crates/engish/
[Minimum Supported Rust Version]: https://img.shields.io/badge/Rust-1.56.0-blue?color=fc8d62&logo=rust
[Rust]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1560-2021-10-21
[Code of Conduct]: https://www.rust-lang.org/en-US/conduct.html
[docs]: "https://img.shields.io/docsrs/engish/"
[docs.rs]: "https://docs.rs/prima/latest/engish/"