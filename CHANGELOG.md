# Changelog

## 1.2.0

- Added a `--tail-command` flag to allow for continuous tailing the output of a specified command
- Added a `-p` / `--print` flag to print the output directly to `stdout`
- Added `man` pages

## 1.1.0 - 24.07.23

**Core**

- `less` now starts with the `--ignore-case` and `--RAW-CONTROL-CHARS` flags
- Added support for reading from `stdin`
- Added a `--version` flag

## 1.0.0 - 11.07.23

**Core**

- `tailspin` has been rewritten in Rust

**Theming**

- All highlight groups are now customizable

## 0.1.1 - 27.02.23

**Bugfixes**

- Fixed a bug that would occasionally lead to temp files not being cleaned up

**Dependencies**

- Bump Go from 1.19 to 1.20
- Bump bubbletea from 0.22.0 to 0.23.2
- Bump golang.org/x/text from 0.3.7 to 0.3.8
- Bump github.com/spf13/cobra from 1.5.0 to 1.6.1

## 0.1.0 - 25.09.22

**Initial release**

`tailspin` is a command line utility for viewing and `tail`-ing log files
