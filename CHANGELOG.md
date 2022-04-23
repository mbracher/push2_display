# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), 
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

-

## [0.2.0] - 2022-04-23

### Changed

- not depending on alpha versions of embedded-graphics anymore.
- updated hello example in README
- added projects using push2_display section in README

## [1.0.0-alpha.3] - 2022-04-22

### Changed

- Updated embedded-graphics-core to 0.3.3
- Updated rusb to 0.9.1
- Updated thiserror to 1.0.30
- For hello example, update embedded-graphics to 0.7.1

## [1.0.0-alpha.2] - 2021-02-22

### Changed

- Updated embedded-graphics-core to 0.2.0
- Allocate framebuffer on heap instead of stack
- For hello example, update embedded-graphics to 0.7.0-alpha.3

## [1.0.0-alpha.1]

### Changed

- Replace embedded-graphics dependency with embedded-graphics-core 0.1.1
- For hello example, update embedded-graphics to 0.7.0-alpha.2

### Fixed

- License header in readme template

## [0.1.1] - 2020-12-29

### Added

- This changelog
- APACHE and MIT license files

### Changed

- Fix layout of license section in readme
- In doc and readme add link to photo of hello example on Push2 device
- Add badges to crates.io and docs.rs in readme template

## [0.1.0] - 2020-12-28

### Added

- Initial version implementing Embedded-graphics 0.6.2 DrawTarget trait for for the Ableton Push2 display
- Very basic hello example
- Photo of hello example on the Ableton Push2 display

[unreleased]: https://github.com/mbracher/push2_display/compare/0.2.0...HEAD
[0.2.0]: https://github.com/mbracher/push2_display/compare/1.0.0-alpha.3...0.2.0
[1.0.0-alpha.3]: https://github.com/mbracher/push2_display/compare/1.0.0-alpha.2...1.0.0-alpha.3
[1.0.0-alpha.2]: https://github.com/mbracher/push2_display/compare/1.0.0-alpha.1...1.0.0-alpha.2
[1.0.0-alpha.1]: https://github.com/mbracher/push2_display/compare/0.1.1...1.0.0-alpha.1
[0.1.1]: https://github.com/mbracher/push2_display/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/mbracher/push2_display/releases/tag/0.1.0