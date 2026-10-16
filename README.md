# bcd_time_stamp

A dependency-free Rust library with optional `no_std` support for packed BCD clock
values and strongly typed second/millisecond timestamps. Rust 1.85 or later.
The clock and timestamp operations do not allocate or access a platform clock
or timezone database.

## Installation

```toml
[dependencies]
bcd_time_stamp = "0.1"
```

## Features

Default features are `std` and `alloc`. `std` also enables `alloc`.
All current APIs are available in every configuration; enabling `alloc` makes
allocation support available internally but does not introduce allocations.

For `no_std` without allocation support:

```toml
[dependencies]
bcd_time_stamp = { version = "0.1", default-features = false }
```

For `no_std` with allocation support:

```toml
[dependencies]
bcd_time_stamp = { version = "0.1", default-features = false, features = ["alloc"] }
```

## Usage

```rust
use bcd_time_stamp::{BcdTimeStamp, TimeStamp, MilliTimeStamp};

let mut time = BcdTimeStamp::from_bcd(0x2359).unwrap();
time += 2; // minutes, wrapping at midnight
assert_eq!(time.timestamp, 0x0001);
time -= 2;
assert_eq!(time.hour(), 0x23); // BCD, not decimal 23
assert_eq!(time.minute(), 0x59); // BCD, not decimal 59
assert!(BcdTimeStamp::from_bcd(0x2460).is_none());

let seconds = TimeStamp(3600) + 60;
assert_eq!(BcdTimeStamp::from(seconds).timestamp, 0x0101);
let millis = MilliTimeStamp(1000) + 500;
assert_eq!(millis.0, 1500);
```

## Semantics and compatibility

- `BcdTimeStamp` stores `0xHHMM` in a `u16`. Valid times are `00:00` through
  `23:59`. `new()` and `default()` create midnight.
- `from_bcd()` validates every digit and the clock range. `From<u16>`, the public
  `timestamp` field, and `DerefMut` provide **unchecked** raw access for backwards
  compatibility. Use `is_valid()` after raw mutation. Arithmetic and `increment()`
  require valid input; results for invalid input are unspecified.
- BCD `+`, `-`, `+=`, and `-=` accept `u16` minutes and wrap within 24 hours,
  including operands up to `u16::MAX`. `increment()` adds one minute.
- `hour()` and `minute()` return BCD bytes. `MINUTE_MASK` selects the minute byte;
  the original misspelled `MINUET_MASK` remains available.
- `TimeStamp` stores seconds; `MilliTimeStamp` stores milliseconds. Both support
  arithmetic with `u64` or their own type, with ordinary `u64` overflow behavior
  (panic when overflow checks are enabled, wrap otherwise). Mixed-unit arithmetic
  is deliberately unsupported. Both are also available under `alias`.
- Converting seconds to BCD discards seconds and whole days. Unix timestamps
  produce UTC time of day. No dates, leap seconds, or local timezones are modeled.

## Development

Run from this independent crate directory (no target override is needed):

```sh
cargo fmt --all -- --check
cargo test
cargo test --release
cargo test --no-default-features
cargo test --no-default-features --features alloc
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
cargo package
cargo publish --dry-run
```

For an embedded compilation check, install your target and run, for example:
`cargo check --no-default-features --target riscv32imafc-unknown-none-elf --lib`.

The tests cover all valid clock times, all raw BCD encodings, boundary timestamps,
and arithmetic. CI also checks Rust 1.85, stable Rust, documentation, packaging,
and a bare-metal RISC-V target.

## Release

Review the version and changelog, confirm license and crates.io name availability,
then run the checks above. Authenticate with crates.io and run `cargo publish`
when ready. Repository metadata can be added when a public repository exists.

## License

Copyright (c) 2026 bcd_time_stamp contributors.

Licensed under the GNU Lesser General Public License, version 3.0 or later
(SPDX: `LGPL-3.0-or-later`). See `LICENSE` for the LGPL terms and `COPYING`
for the GNU GPL version 3 terms incorporated by the LGPL.
