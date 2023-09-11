<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/jaynewey/baskerville/main/static/logo-dark.svg?raw=true" width="50%">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/jaynewey/baskerville/main/static/logo-light.svg?raw=true" width="50%">
    <img src="https://raw.githubusercontent.com/jaynewey/baskerville/main/static/logo-light.svg?raw=true" width="50%">
  </picture>

---

[![Crates.io](https://img.shields.io/crates/v/baskerville)](https://crates.io/crates/baskerville)
[![docs.rs](https://img.shields.io/docsrs/baskerville)](https://docs.rs/baskerville/)
![GitHub](https://img.shields.io/github/license/jaynewey/baskerville)

Infer and validate data-type schemas in Rust and Python.

[Rust](https://github.com/jaynewey/baskerville)
•
[Python](https://github.com/jaynewey/baskerville-py)

</div>

## Installation

```
cargo add baskerville
```

## Example

```csv
# mascots.csv
Name,LOC,Species
Ferris,42,Crab
Corro,7,Urchin
```

```rust
use baskerville::{infer_csv_with_options, CsvInput, InferOptions};

fn main() {
    let fields = infer_csv_with_options(
        CsvInput::Path("mascots.csv"),
        &mut InferOptions {
            has_headers: true,
            ..InferOptions::default()
        },
    )
    .unwrap();
    println!("{fields}");
}
```

Output:

```
╭──────┬─────────┬─────────╮
│ Name │ LOC     │ Species │
├──────┼─────────┼─────────┤
│ Text │ Integer │ Text    │
│      │ Float   │         │
│      │ Text    │         │
╰──────┴─────────┴─────────╯
```

## Contributing

<!-- TODO: add "pre-commit checklist" when CI is set up -->

### Versioning

The repo bases versioning from [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
