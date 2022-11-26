<p align="center">
  <img src="assets/tailspin.png" width="230"/>
</p>

# 

<p align="center">
A log file highlighter
</p>

<p align="center">
  <img src="assets/example.png" width="700"/>
</p>

## Overview

`tailspin` is a command line tool for viewing (and `tail`-ing) log files. It highlights important keywords to make
navigating log files easier.

## Installing

### Homebrew

```console
# Install
brew install bensadeh/tailspin/tailspin

# View log file
spin [file]

# Tail log file
spin -f [file]
```
## Highlight Groups

### Dates
- `2022-08-29 08:11:36`
- `2022-09-09 11:48:34,534`

### Special Keywords
- `true`
- `false`
- `null`

### URLs

Individual highlighting for each part of a URL

| Component         | Example        |
|-------------------|----------------|
| scheme            | `http`/`https` |
| domain            | `google.com`   |
| subdomain         | `/search`      |
| search parameters | `/?key=value`  |


### Severity and Log levels
- `TRACE`
- `INFO`
- `WARN`
- `ERROR`

### Numbers
- `100`
- `200`

### UUIDs and GUIDs
- `123e4567-e89b-12d3-a456-426614174000`

## See also

* [lnav](https://github.com/tstack/lnav)
* [grc](https://github.com/garabik/grc)

## Under the hood

`tailspin` uses:

* [`less`](http://greenwoodsoftware.com/less/)
* [Bubble Tea](https://github.com/charmbracelet/bubbletea) for handling control over to `less`
* [cobra](https://github.com/spf13/cobra) for the CLI
