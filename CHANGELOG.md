# Changelog

<!-- next-header -->
## Unreleased - ReleaseDate

### Added

- `Base64Slice`, the borrowed counterpart of `Base64Vec`, analogous to `str`
  and `String`.

### Changed

- **Breaking:** `Base64Vec` now dereferences to `Base64Slice` instead of
  `Vec<u8>`, and no longer implements `DerefMut`. `Base64Slice` dereferences
  to `[u8]`, so slice methods such as `len` and indexing still work. To modify
  the bytes, use the public field (`.0`) or `AsMut<[u8]>`.
- **Breaking:** The `base64` feature no longer implies `alloc`. `Base64Slice`
  is now available without `alloc`, but `Base64Vec` and `ParseBase64Error`
  require both `base64` and `alloc`. `alloc` is enabled by default; if you use
  `default-features = false` with `features = ["base64"]`, add `"alloc"` to keep
  `Base64Vec`.
- `Base64Slice` and `Base64Vec` now format with `Display` and `Debug` without
  allocating an intermediate `String`.

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
[0.1.1]: https://github.com/oxidecomputer/byte-wrapper/releases/tag/byte-wrapper-0.1.1
[0.1.0]: https://github.com/oxidecomputer/byte-wrapper/releases/tag/byte-wrapper-0.1.0
