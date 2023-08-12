# Changelog

## 1.4.0 - 12.08.23

- Added `-t`/`--tail` flag to start reading from the end of a file.
- Fixed a bug where opening a folder would include hidden files
- Improved inital output when watching folders
- Improved output when trying to open a file or folder which doesn't exist

## 1.3.0 - 31.07.23

- Added support for tailing folders
- Changed behavior: `tailspin` will now print to `stdout` by default if used in a pipe. For
  example: `echo "hello null" | spin`
  will output a syntax highlighted "hello null" directly to `stdout` instead of via the pager less. The change will make
  it easier to use tailspin in scripting and piping.
- The `--tail-command` flag has been renamed to `--follow-command`

## 1.2.1 - 26.07.23

- Run `less` with environment variable `LESSSECURE=1`

## 1.2.0 - 26.07.23

- Added a `--tail-command` flag to allow for continuous tailing the output of a specified command
- Added a `-p` / `--print` flag to print the output directly to `stdout`
- Added `man` pages
- Added shell completions for `bash`, `zsh` and `fish`

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
