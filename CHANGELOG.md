# Changelog

<!-- next-header -->
## Unreleased - ReleaseDate

## [0.2.1] - 2026-10-21

### Fixed

- For JSON schemas, the `x-rust-type` correctly sets version 0.2.0.

## [0.2.0] - 2026-10-21

### Added

- `Base64Slice`, the borrowed counterpart of `Base64Vec`, analogous to `str`
  and `String`.
- `HexSlice` and `HexVec`, a borrowed/owned pair of hex types for
  variable-length bytes, analogous to `str` and `String`.

### Changed

- `Base64Vec` now dereferences to `Base64Slice` instead of `Vec<u8>`, and no longer
  implements `DerefMut`. `Base64Slice` dereferences to `[u8]`, so slice methods such
  as `len` and indexing still work. To modify the bytes, use the public field (`.0`)
  or `AsMut<[u8]>`.

- The `base64` feature no longer implies `alloc`. `Base64Slice` is now available without
  `alloc`, but `Base64Vec` requires both `base64` and `alloc`.

  `alloc` is enabled by default; if you use `default-features = false` with
  `features = ["base64"]`, add `"alloc"` to keep using `Base64Vec`.

- `ParseHexError` has a new `OddLength` variant, returned when parsing a
  `HexVec` from a string with an odd number of characters. This is a breaking
  change for code that matches exhaustively on `ParseHexError`.

### Fixed

- `Base64Vec` can now be deserialized from CBOR byte strings longer than 4096
  bytes, as well as from indefinite-length (chunked) byte strings. Previously,
  these failed with "invalid type: bytes, expected bytes".

## [0.1.1] - 2026-10-20

### Changed

- Updated the `base64` dependency to 0.23.

### Fixed

- The `base64` feature now builds on `no_std` targets. Previously, the `base64`
  dependency was built with its `std` feature.

## [0.1.0] - 2026-03-05

Initial release, with support for fixed-length hex arrays and variable-length base64 vectors.

<!-- next-url -->
[0.2.1]: https://github.com/oxidecomputer/byte-wrapper/releases/tag/byte-wrapper-0.2.1
[0.2.0]: https://github.com/oxidecomputer/byte-wrapper/releases/tag/byte-wrapper-0.2.0
[0.1.1]: https://github.com/oxidecomputer/byte-wrapper/releases/tag/byte-wrapper-0.1.1
[0.1.0]: https://github.com/oxidecomputer/byte-wrapper/releases/tag/byte-wrapper-0.1.0
