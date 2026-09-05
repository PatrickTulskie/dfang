# Changelog

`dfang` and `rfang` are versioned and released together, so one entry covers both.

Dates are the crates.io publish dates. Entries before this file existed were
reconstructed from the git history and the release tags.

## Unreleased

### Added

- `--line-buffered` flushes output after every line instead of every block,
  for `tail -f` style input or an interactive reader downstream.

### Changed

- Piped input is handled line by line as it arrives instead of being read to
  the end first, so a `tail -f` can be piped in and memory is bounded by the
  longest line rather than the whole input.
- Piped input is written through a buffer instead of flushing stdout on every
  line, which roughly doubles throughput on large inputs. Output is unchanged.
- A downstream reader closing early (`dfang < iocs.txt | head -1`) now ends the
  run quietly instead of panicking with a broken-pipe message.
- Both crates declare a minimum supported Rust version of 1.78, tested in CI.
- Readme points at the Homebrew tap, which has carried a formula for both
  binaries since 0.3.0.

## 0.3.0 - 2026-08-09

### Added

- `dfang` and `rfang` now ship a library alongside the binary, so `defang` and
  `refang` can be called from Rust instead of shelled out to. Neither library
  has dependencies and neither does any I/O.
- `fang-conformance`, an unpublished workspace member holding one corpus of
  defang/refang pairs asserted in both directions, so the two crates can't drift
  apart on the tokens they exchange.

### Changed

- Readme documents the library entry points, and its code fences carry language
  hints.

## 0.2.0 - 2026-08-08

### Added

- Prebuilt binaries for macOS, Linux, and Windows attached to each release, so
  installing no longer requires a Rust toolchain.

### Changed

- Every IOC on a line is defanged, not just the first kind matched. A line
  carrying a URL and an email now comes out with both escaped.
- A URL scheme keeps the case it arrived in, so `HTTP://` defangs to `HXXP://`
  and refangs back to `HTTP://` rather than being lowercased along the way.

### Removed

- The `regex` and `lazy_static` dependencies, replaced with plain string
  scanning. Both crates now build with no dependencies at all.
- A placeholder root binary that nothing built.

## 0.1.5 - 2023-11-16

### Changed

- Pipe detection uses `std::io::IsTerminal` instead of the unmaintained `atty`
  crate.

## 0.1.4 - 2023-01-30

### Added

- `--help` output shows the version.

## 0.1.3 - 2023-01-30

### Added

- First round of tests.

### Changed

- Fewer allocations in the string replacement path.

## 0.1.2 - 2023-01-27

### Added

- Input can be piped in from another command, so `pbpaste | dfang | pbcopy`
  works.

## 0.1.1 - 2023-01-25

### Added

- Multiple arguments in one invocation, each defanged on its own line.

### Fixed

- Argument handling that mishandled anything past the first input.

## 0.1.0 - 2023-01-25

Initial release of `dfang` and `rfang`.
