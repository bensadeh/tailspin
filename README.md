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

## Overview

`tailspin` is a command line tool for viewing (and `tail`-ing) log files. It highlights important keywords to make
navigating log files easier.

`tailspin` is fast and easy to customize. It uses `less` under the hood to provide scrollback, search and filtering.

## Installing

```console
# Install
cargo install tailspin

# View log file
spin [file]

# Tail log file
spin [file] -f
```

> **Note** 
> When installing via cargo, make sure that `$HOME/.cargo/bin` is in your `PATH` environment variable

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

## Customizing highlight groups

### Overview

`tailspin` uses a single `config.toml` file to configure all highlight groups. When customizing highlights it is advised
to start with the `tailspin generate-config` command to place a `config.toml` with default options
in `~/.config/tailspin`.

To disable a highlight group, either comment it out or delete it.

Highlights have the following shape:

```toml
style = { fg = "color", bg = "color", italic = false, bold = false, underline = false }
```

### Adding keywords

To add custom keywords, either include them in the list of keywords or add new entries:

```toml
[[groups.keywords]]
words = ['MyCustomKeyword']
style = { fg = "green" }

[[groups.keywords]]
words = ['null', 'true', 'false']
style = { fg = "red", italic = true }
```

## Search and Filtering

`tailspin` uses `less` as its pager to view the highlighted log files.

In `less`, use <kbd>/</kbd> followed by your search query. For example, `/ERROR` finds the first occurrence of
**ERROR**. After the search, <kbd>n</kbd> finds the next instance, and <kbd>N</kbd> finds the previous instance.

`less` allows filtering lines by a keyword, using <kbd>&</kbd> followed by the pattern. For instance, `&ERROR` shows
only lines with **ERROR**.

To only show lines containing either `ERROR` or `WARN`, use a regular expression: `&\(ERROR\|WARN\)`.

To clear the filter, use <kbd>&</kbd> with no pattern.

## Settings

```console
# Commands
generate-config    Create a custom config file at ~/.config/tailspin

# Options
-f, --follow       Follow (tail) the contents of the file
    --config       Provide a custom path configuration file
```



