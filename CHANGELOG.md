# Changelog

## 3.0.0

- Process highlighter has updated default styling and matches processes with parenthesis
- UUID highlighter now highlights numbers and letters in individual styling
- Added a new highlighter for highlighting pointers
- Re-Enabled parallel processing from `stdin`
- Renamed `--follow-command` to `--listen-command`
- Removed `bucket-size` flag

## 2.4.0

- Added a regexp highlighter with support for one capture group
- Changed the behavior of processing lines from `stdin` to be sequential for better stability

## 2.3.0

- `tailspin` now uses multiple threads to process lines in parallel
- Added `--bucket-size` flag to configure the number of lines to process in parallel
- Changed `-t,--tail` flag to `-e,--start-at-end` to avoid confusion with `tail -f`

## 2.2.0

- Added flags for setting simple highlights on the fly, for example: (`tspin --words-red popcorn,movie`)
- Properly flatten and merge keywords to improve regex performance
- Binaries are now added to the GitHub Release (Thanks @ecarrara and @supleed2)
- Added `HEAD` HTTP method to the REST keywords (Thanks @mkogan1)
- Fixed a bug where the message `Failed to open file with less: Exit code 0` would show after exiting `less`

## 2.1.0

- Fixed a bug where opening empty files would hang forever
- Look for config file in `USERPROFILE` and `$HOME` instead of just `$HOME`
- Added flags for disabling builtin keywords
- Process names with dashes are now highlighted properly
- Better error messages when `less` is not found

## 2.0.0

- Changed the binary name from `spin` to `tspin`

This is a symbolic release to settle on a new binary name with fewer conflicts. Both `tailspin` and `spin` already exist
as binaries in different systems and distributions. `tspin` is a short and unique name that is unlikely to conflict with
other binaries.

## 1.6.1

- Fixed a bug where the `--print` flag would occasionally cause a panic

## 1.6.0

- Added new highlight group under Keywords highlighter: HTTP methods
- Added option for adding a border to keywords highlighter
- Disable highlights with `disable` for all highlight groups except Keywords
- Simplified the configuration file format
- Date and time can be configured to be hidden

## 1.5.1

- Update man pages

## 1.5.0 - 16.09.23

- Errors are now printed to `stderr` instead of `stdout`
- Date highlighter now supports different highlights for date and time segment
- Added Key Value highlighter
- Added unix process highlighter

## 1.4.0 - 12.08.23

- Added `-t`/`--tail` flag to start reading from the end of a file.
- Fixed a bug where opening a folder would include hidden files
- Improved initial output when watching folders
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
