# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

## [0.4.0]

- [#20] - Support wildcards that are not preceded by a "/" (`"/foo_*rest"` matches `"foo_bar/baz"`).

### Changed

[#20]: https://github.com/viz-rs/path-tree/pull/20

## [0.3.1] - 2022-07-15

- [#19] - PathTree initializes with an empty path. Thanks to @adriangb.

### Changed

[#19]: https://github.com/viz-rs/path-tree/pull/19

## [0.3.0] - 2022-05-28

### Changed

- [#17] - Add basic fuzzing. Thanks to @Txuritan.
- [#16] - Replace the Node.indices String with a Vec<char>, this avoids the expensive code point decoding in position. Thanks to @Txuritan.

[#17]: https://github.com/viz-rs/path-tree/pull/17
[#16]: https://github.com/viz-rs/path-tree/pull/16

## [0.2.2] - 2021-10-24

### Fixed

- [#8] - Thanks to @josalmi

[#8]: https://github.com/viz-rs/path-tree/pull/8

## [0.2.1] - 2021-09-11

### Changed

- Clippy Fixed

## [0.2.0] - 2021-09-03

### Changed

- Route finding optimizations. Thanks to @Txuritan.

## [0.1.12] - 2020-09-29

### Fixed

- Find byte index, use `str#find` instead of `position`. Thanks to @asaaki.

## [0.1.11] - 2020-06-21

### Changed

- Readme

## [0.1.10] - 2020-06-20

### Changed

- `const fn`

## [0.1.9] - 2019-11-05

### Added

- Benchmark data.
- A lifetime for result.

## Changed

- Dont use unsafe code.
- Dont need mut.

## [0.1.4] - 2019-03-18

### Changed

- Tuple struct for PathTree.

[Unreleased]: https://github.com/viz-rs/path-tree/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/viz-rs/path-tree/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/viz-rs/path-tree/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/viz-rs/path-tree/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/viz-rs/path-tree/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/viz-rs/path-tree/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/viz-rs/path-tree/compare/v0.1.12...v0.2.0
[0.1.12]: https://github.com/viz-rs/path-tree/compare/v0.1.11...v0.1.12
[0.1.11]: https://github.com/viz-rs/path-tree/compare/v0.1.10...v0.1.11
[0.1.10]: https://github.com/viz-rs/path-tree/compare/v0.1.9...v0.1.10
[0.1.9]: https://github.com/viz-rs/path-tree/compare/v0.1.4...v0.1.9
[0.1.4]: https://github.com/viz-rs/path-tree/releases/tag/v0.1.4
