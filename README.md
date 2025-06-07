# Delta-Encoding library

[![GitHub repo](https://img.shields.io/badge/github-nyurik/delta--encoding-8da0cb?logo=github)](https://github.com/nyurik/delta-encoding)
[![crates.io version](https://img.shields.io/crates/v/delta-encoding)](https://crates.io/crates/delta-encoding)
[![crate usage](https://img.shields.io/crates/d/delta-encoding)](https://crates.io/crates/delta-encoding)
[![docs.rs status](https://img.shields.io/docsrs/delta-encoding)](https://docs.rs/delta-encoding)
[![crates.io license](https://img.shields.io/crates/l/delta-encoding)](https://github.com/nyurik/delta-encoding/blob/main/LICENSE-APACHE)
[![CI build status](https://github.com/nyurik/delta-encoding/actions/workflows/ci.yml/badge.svg)](https://github.com/nyurik/delta-encoding/actions)
[![Codecov](https://img.shields.io/codecov/c/github/nyurik/delta-encoding)](https://app.codecov.io/gh/nyurik/delta-encoding)


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
use delta_encoding::{DeltaEncoderExt, DeltaDecoderExt};

pub fn main() {
  let data = vec![1, 2, 5, 4, 2];

  // Delta-encode without consuming, and without making a vector copy
  let encoded: Vec<i64> = data.iter().copied().deltas().collect();
  assert_eq!(encoded, vec![1, 1, 3, -1, -2]);

  // Consume and delta-encode
  let encoded: Vec<i64> = data.into_iter().deltas().collect();
  assert_eq!(encoded, vec![1, 1, 3, -1, -2]);

  let data = vec![1, 1, 3, -1, -2];

  // Delta-decode without consuming, and without making a vector copy
  let decoded: Vec<i64> = data.iter().copied().original().collect();
  assert_eq!(decoded, vec![1, 2, 5, 4, 2]);

  // Consume and delta-decode
  let decoded: Vec<i64> = data.into_iter().original().collect();
  assert_eq!(decoded, vec![1, 2, 5, 4, 2]);
}
```

## Development

* This project is easier to develop with [just](https://github.com/casey/just#readme), a modern alternative to `make`.
  Install it with `cargo install just`.
* To get a list of available commands, run `just`.
* To run tests, use `just test`.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual-licensed as above, without any
additional terms or conditions.
