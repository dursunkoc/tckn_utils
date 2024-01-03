<h1 align="center">tckn_utils</h1>
<div align="center">
  <strong>
    TCKN Generator & VALIDATOR for Rust
  </strong>
</div>

<br />

<div align="center">
  <a href="https://crates.io/crates/tckn_utils">
    <img src="https://img.shields.io/crates/v/tckn_utils.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <a href="https://crates.io/crates/tckn_utils">
    <img src="https://img.shields.io/crates/d/tckn_utils.svg?style=flat-square"
      alt="Download" />
  </a>
  <a href="https://docs.rs/tckn_utils">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/tckn_utils">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/dursunkoc/tckn_utils/releases">
      Releases
    </a>
  </h3>
</div>

## Installation

With [cargo add][cargo-add] installed run:

```sh
$ cargo add tckn_utils
```

[cargo-add]: https://github.com/killercup/cargo-edit

## Generate & Validate TCKN

```rs

use tckn_utils::{generate, validate};

fn main() {
    let tckn = generate();
    let valid = validate(tckn.as_str());
    println!("TCKN: {tckn} => validate: {valid}");
}

```

## Contributing

Want to join us? Look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

[good-first-issue]: https://github.com/dursunkoc/tckn_utils/labels/good%20first%20issue
[help-wanted]: https://github.com/dursunkoc/tckn_utils/labels/help%20wanted

## License

<sup>
Licensed under either of <a href="LICENSE.md">MIT license</a>
</sup>

