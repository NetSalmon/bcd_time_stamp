# Changelog

## 0.1.0

- Enable `std` and `alloc` by default; support disabling both or using only `alloc`.

- Extract the independent, dependency-free `no_std` crate.
- Preserve BCD and typed timestamp arithmetic APIs.
- Add checked BCD construction, validation, and the corrected `MINUTE_MASK` name.
- Document units, conversion, validation, and overflow behavior.
- Add exhaustive clock/encoding tests and release checks.
