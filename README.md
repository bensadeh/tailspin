<p align="center">
  <img src="assets/tailspin.png" width="230"/>
</p>

#                                                                                                                                                                                                                                                          

<p align="center">
A log file highlighter
</p>

<p align="center">
  <img src="assets/main.png" width="700"/>
</p>

### Features

- 🪵 View (or `tail`) any log file of any format
- 🍰 No setup or config required
- 🌈 Highlight numbers, dates, IP-addresses, UUIDs, URLs and more
- ⚙️ All highlight groups are customizable
- 🔍 Uses `less` under the hood to provide **scrollback**, **search** and **filtering**

#

### Table of Contents

* [Overview](#overview)
* [Installing](#installing)
* [Highlight Groups](#highlight-groups)
* [Customizing Highlight Groups](#customizing-highlight-groups)
* [Using the pager `less`](#using-the-pager-less)
* [Settings](#settings)

***

## Overview

`tailspin` works by reading through a log file line by line, running a series of regexes
against each line. The regexes recognize patterns like dates, numbers, severity
keywords and more.

`tailspin` does not make any assumptions on the format or position of the items it wants to highlight. For this reason,
it requires no configuration or setup and will work predictably regardless of the format the log file is in.

## Installing

### Cargo

```console
# Install
cargo install tailspin

# View log file
spin [file]
```

> **Note**
> When installing via cargo, make sure that `$HOME/.cargo/bin` is in your `PATH` environment variable

### Debian

```console
# Install
apt install tailspin

# View log file
tailspin [file]
```

> **Note**
> Because of a name collision with another `apt` package, the binary name on Debian is `tailspin`

## Highlight Groups

### Dates

<p align="center">
  <img src="assets/examples/dates.png" width="600"/>
</p>

### Keywords

<p align="center">
  <img src="assets/examples/keywords.png" width="600"/>
</p>

### URLs

<p align="center">
  <img src="assets/examples/urls.png" width="600"/>
</p>

### Numbers

<p align="center">
  <img src="assets/examples/numbers.png" width="600"/>
</p>

### IP Addresses

<p align="center">
  <img src="assets/examples/ip.png" width="600"/>
</p>

### Quotes

<p align="center">
  <img src="assets/examples/quotes.png" width="600"/>
</p>

### Unix file paths

<p align="center">
  <img src="assets/examples/paths.png" width="600"/>
</p>

### UUIDs

<p align="center">
  <img src="assets/examples/uuids.png" width="600"/>
</p>

## Customizing Highlight Groups

### Overview

`tailspin` uses a single `config.toml` file to configure all highlight groups. When customizing highlights, it is
advised to start with the `--create-default-config ` flag to place a `config.toml` with default options
in `~/.config/tailspin`.

To disable a highlight group, either comment it out or delete it.

Highlights have the following shape:

```toml
style = { fg = "color", bg = "color", italic = false, bold = false, underline = false }
```

### Adding Keywords

To add custom keywords, either include them in the list of keywords or add new entries:

```toml
[[groups.keywords]]
words = ['MyCustomKeyword']
style = { fg = "green" }

[[groups.keywords]]
words = ['null', 'true', 'false']
style = { fg = "red", italic = true }
```

## Using the pager `less`

### Overview

`tailspin` uses `less` as its pager to view the highlighted log files. You can get more info on `less` via the **man**
command (`man less`) or by hitting the <kbd>h</kbd> button to access the help screen.

### Navigating

Navigating within `less` uses a set of keybindings that may be familiar to users of `vim` or other `vi`-like
editors. Here's a brief overview of the most useful navigation commands:

- <kbd>j</kbd>/<kbd>k</kbd>: Scroll one line up / down
- <kbd>d</kbd>/<kbd>u</kbd>: Scroll one half-page up / down
- <kbd>g</kbd>/<kbd>G</kbd>: Go to the top / bottom of the file

### Follow mode

When you run `tailspin` with the `-f` or `--follow` flag, it will scroll to the bottom and print new lines to the screen
as they're added to the file.

To stop following the file, interrupt with <kbd>Ctrl + C</kbd>. This will stop the tailing, but keep the
file open, allowing you to review the existing content.

To resume following the file from within `less`, press <kbd>Shift</kbd>+<kbd>f</kbd>.

### Search

Use <kbd>/</kbd> followed by your search query. For example, `/ERROR` finds the first occurrence of
**ERROR**.

After the search, <kbd>n</kbd> finds the next instance, and <kbd>N</kbd> finds the previous instance.

### Filtering

`less` allows filtering lines by a keyword, using <kbd>&</kbd> followed by the pattern. For instance, `&ERROR` shows
only lines with **ERROR**.

To only show lines containing either `ERROR` or `WARN`, use a regular expression: `&\(ERROR\|WARN\)`.

To clear the filter, use <kbd>&</kbd> with no pattern.

## Settings

```console
-f, --follow                 Follow (tail) the contents of the file
-p, --print                  Print the output to stdout
-c, --config-path PATH       Path to a custom configuration file
-t, --tail-command 'CMD'     Tails the output of the provided command
    --create-default-config  Generate a new configuration file
    --show-default-config    Print the default configuration
```



