[![geo on Crates.io](https://img.shields.io/crates/v/delta-encoding.svg)](https://crates.io/crates/delta-encoding)
[![Coverage Status](https://coveralls.io/repos/github/nyurik/delta-encoding/badge.svg)](https://coveralls.io/github/nyurik/delta-encoding)
[![Documentation](https://docs.rs/delta-encoding/badge.svg)](https://docs.rs/delta-encoding)

# Delta-Encoding library

A simple library for encoding and decoding a stream of values as delta-encoded.  For example, if you have a stream of values like this:

```text
1, 3, 2, 4, 5
```

the delta-encoded stream would be:

```text
1, 2, -1, 2, 1
```

## Usage

```rust
use delta_encoding::DeltaEncoderExt;

pub fn main() {
    let data = vec![1, 2, 5, 4, 2];
    let encoded: Vec<i64> = data.into_iter().to_deltas().collect();

    assert_eq!(encoded, vec![1, 1, 3, -1, -2]);
}
```

## Development
All of these must succeed:
```bash
cargo test    # Testing
cargo bench   # Benchmarking
cargo fmt     # Code format
cargo clippy  # Code lints
```

## TODO
* implement a non-consuming iterator, e.g. `data.iter().as_deltas().collect()`
