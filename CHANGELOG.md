# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com),
and this project adheres to [Semantic Versioning](https://semver.org).

<!--
Types of changes:
- `Added` for new features;
- `Changed` for changes in existing functionality;
- `Deprecated` for soon-to-be removed features;
- `Removed` for now removed features;
- `Fixed` for any bug fixes;
- `Security` in case of vulnerabilities.
-->

<!-- next-header -->
## [Unreleased]

## [0.3.0] - 2024-05-20

### Fixed

- Implementation of `two_sum` and, consequently, `KahanBabuskaNeumaier`.

### Added

- More tests and benchmarks.

## [0.2.0] - 2024-05-14

### Added

- Benchmarks.
- More tests.

### Changed

- `KahanBabuska: std::iter::Sum` and `KahanBabuskaNeumaier: std::iter::Sum` now require an
  `AddAssign` bound instead of `Add`.

  Furthermore, the implementation uses a `for` loop instead of `Iterator::fold`, which improves codegen
  (smaller assembly, though similar performance).

## [0.1.0] - 2024-05-14

### Added

- `two_sum` and `fast_two_sum` algorithms.
- `KahanBabuska` and `KahanBabuskaNeumaier` summation.

<!-- next-url -->
[Unreleased]: https://github.com/FedericoStra/compensated-summation/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/FedericoStra/compensated-summation/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/FedericoStra/compensated-summation/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/FedericoStra/compensated-summation/releases/tag/v0.1.0
