# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- ✨ add word source management(pr [#52])
- ✨ add colorful crate to the project(pr [#53])

## [0.1.13] - 2025-04-05

### Added

- ✨ add shuffle depth option for word chain(pr [#46])
- ✨ add solution module for word chain puzzle(pr [#47])
- ✨ add progress bar to solution generation(pr [#48])

### Changed

- ♻️ refactor(cli)-streamline command structure and logic(pr [#49])
- ♻️ refactor(main)-update default source file name(pr [#50])
- Add-word-count-sorting-to-solutions(pr [#51])

## [0.1.12] - 2025-04-03

### Added

- ✨ add generate command(pr [#43])
- ✨ add bare option for letter output(pr [#44])
- ✨ enhance letter validation and output formatting(pr [#45])

## [0.1.11] - 2025-04-02

### Added

- ✨ add custom error handling with thiserror(pr [#42])

## [0.1.10] - 2025-04-02

### Changed

- ♻️ refactor(cli)-simplify letter handling in CmdSolutions and CmdSolve(pr [#41])

## [0.1.9] - 2025-04-02

### Added

- ✨ add solutions command for word chain generation(pr [#35])

### Changed

- restructure project into a workspace(pr [#33])
- Add-max-chain-length-option-for-solutions(pr [#37])
- 🔧 chore(workspace)-update Cargo.toml for workspace configuration(pr [#38])
- ♻️ refactor(workspace)-consolidate workspace structure(pr [#39])
- ♻️ refactor(project)-restructure project to remove CLI and workspace(pr [#40])

### Security

- Dependencies: update rust crate clap to v4.5.35(pr [#36])

## [0.1.8] - 2025-03-31

### Changed

- 🔧 chore(dependencies)-format dependencies and add metadata(pr [#31])
- ♻️ refactor(main)-update default source directory path(pr [#32])

## [0.1.7] - 2025-03-31

### Added

- ✨ add shuffle struct for word shuffling(pr [#30])

## [0.1.6] - 2025-03-31

### Added

- ✨ add twice shuffle option to command(pr [#29])

## [0.1.5] - 2025-03-31

### Added

- ✨ add randomness to word chain building(pr [#26])
- ✨ add shuffle iterations option(pr [#27])
- ✨ add shuffle count option for word chain building(pr [#28])

## [0.1.4] - 2025-03-27

### Added

- ✨ add configuration file support(pr [#25])

## [0.1.3] - 2025-03-27

### Added

- ✨ add list command for directory files(pr [#24])

## [0.1.2] - 2025-03-27

### Changed

- ✅ test(version)-update version in test cases(pr [#21])
- 🔧 chore(config)-update release.toml for version updates(pr [#22])
- 🔧 chore(config)-update version.trycmd file path(pr [#23])

## [0.1.1] - 2025-03-27

### Added

- ✨ add WeightedWord struct(pr [#17])
- ✨ convert words and letters to lowercase(pr [#18])
- ✨ add subcommands for prepare and solve(pr [#19])

### Changed

- Enable-changelog-replacements(pr [#16])

### Fixed

- deps: update rust crate clap to v4.5.34(pr [#20])

## [0.1.0] - 2025-03-26

### Added

- ✨ add command-line interface with logging support(pr [#10])

### Changed

- 👷 ci(circleci)-update toolkit orb source(pr [#6])
- Add-words-struct-for-word-list-preparation(pr [#7])
- Add-LettersBoxed-struct(pr [#8])
- Add-new-command-line-options-for-word-list(pr [#11])
- ✅ test(cli)-add CLI tests using trycmd(pr [#12])
- ♻️ refactor(main)-enhance words loading mechanism(pr [#13])
- Add-release-flag-parameter-to-CI-configuration(pr [#14])
- 🔧 chore(release)-comment out unused replacements(pr [#15])

### Fixed

- deps: update rust crate log to v0.4.27(pr [#9])

[#6]: https://github.com/jerus-org/slb/pull/6
[#7]: https://github.com/jerus-org/slb/pull/7
[#8]: https://github.com/jerus-org/slb/pull/8
[#9]: https://github.com/jerus-org/slb/pull/9
[#10]: https://github.com/jerus-org/slb/pull/10
[#11]: https://github.com/jerus-org/slb/pull/11
[#12]: https://github.com/jerus-org/slb/pull/12
[#13]: https://github.com/jerus-org/slb/pull/13
[#14]: https://github.com/jerus-org/slb/pull/14
[#15]: https://github.com/jerus-org/slb/pull/15
[#16]: https://github.com/jerus-org/slb/pull/16
[#17]: https://github.com/jerus-org/slb/pull/17
[#18]: https://github.com/jerus-org/slb/pull/18
[#19]: https://github.com/jerus-org/slb/pull/19
[#20]: https://github.com/jerus-org/slb/pull/20
[#21]: https://github.com/jerus-org/slb/pull/21
[#22]: https://github.com/jerus-org/slb/pull/22
[#23]: https://github.com/jerus-org/slb/pull/23
[#24]: https://github.com/jerus-org/slb/pull/24
[#25]: https://github.com/jerus-org/slb/pull/25
[#26]: https://github.com/jerus-org/slb/pull/26
[#27]: https://github.com/jerus-org/slb/pull/27
[#28]: https://github.com/jerus-org/slb/pull/28
[#29]: https://github.com/jerus-org/slb/pull/29
[#30]: https://github.com/jerus-org/slb/pull/30
[#31]: https://github.com/jerus-org/slb/pull/31
[#32]: https://github.com/jerus-org/slb/pull/32
[#33]: https://github.com/jerus-org/slb/pull/33
[#35]: https://github.com/jerus-org/slb/pull/35
[#36]: https://github.com/jerus-org/slb/pull/36
[#37]: https://github.com/jerus-org/slb/pull/37
[#38]: https://github.com/jerus-org/slb/pull/38
[#39]: https://github.com/jerus-org/slb/pull/39
[#40]: https://github.com/jerus-org/slb/pull/40
[#41]: https://github.com/jerus-org/slb/pull/41
[#42]: https://github.com/jerus-org/slb/pull/42
[#43]: https://github.com/jerus-org/slb/pull/43
[#44]: https://github.com/jerus-org/slb/pull/44
[#45]: https://github.com/jerus-org/slb/pull/45
[#46]: https://github.com/jerus-org/slb/pull/46
[#47]: https://github.com/jerus-org/slb/pull/47
[#48]: https://github.com/jerus-org/slb/pull/48
[#49]: https://github.com/jerus-org/slb/pull/49
[#50]: https://github.com/jerus-org/slb/pull/50
[#51]: https://github.com/jerus-org/slb/pull/51
[#52]: https://github.com/jerus-org/slb/pull/52
[#53]: https://github.com/jerus-org/slb/pull/53
[Unreleased]: https://github.com/jerus-org/slb/compare/v0.1.13...HEAD
[0.1.13]: https://github.com/jerus-org/slb/compare/v0.1.12...v0.1.13
[0.1.12]: https://github.com/jerus-org/slb/compare/v0.1.11...v0.1.12
[0.1.11]: https://github.com/jerus-org/slb/compare/v0.1.10...v0.1.11
[0.1.10]: https://github.com/jerus-org/slb/compare/v0.1.9...v0.1.10
[0.1.9]: https://github.com/jerus-org/slb/compare/v0.1.8...v0.1.9
[0.1.8]: https://github.com/jerus-org/slb/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/jerus-org/slb/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/jerus-org/slb/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/jerus-org/slb/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/jerus-org/slb/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/jerus-org/slb/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/jerus-org/slb/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/jerus-org/slb/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jerus-org/slb/releases/tag/v0.1.0
