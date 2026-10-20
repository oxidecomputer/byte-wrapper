# Changelog

<!-- next-header -->
## Unreleased - ReleaseDate

### Changed

- Updated the `base64` dependency to 0.23.

### Fixed

- The `base64` feature now builds on `no_std` targets. Previously, the `base64`
  dependency was built with its `std` feature.

## [0.1.0] - 2026-03-05

Initial release, with support for fixed-length hex arrays and variable-length base64 vectors.

<!-- next-url -->
[0.1.0]: https://github.com/oxidecomputer/byte-wrapper/releases/tag/byte-wrapper-0.1.0
