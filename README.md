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
#### Format
```toml
[groups.date]
style = { fg = "magenta" }
```

#### Description
Matches any date in the following formats:
- `YYYY-MM-DD`
- `YYYY-MM-DD HH:MM:SS`
- `YYYY-MM-DD HH:MM:SS,SSS`


### Keywords
#### Format
```toml
[[groups.keywords]]
words = ['DEBUG']
style = { fg = "green" }

[[groups.keywords]]
words = ['null', 'true', 'false']
style = { fg = "red", italic = true }
```
#### Description
The `keywords` is used to highlight any keywords. It is defined as an array of strings 
and can be defined multiple times.


### URLs
#### Format
```toml
http = { faint = true }
https = { bold = true }
host = { fg = "blue", faint = true }
path = { fg = "blue" }
query_params_key = { fg = "cyan" }
query_params_value = { fg = "magenta" }
symbols = { fg = "red" }
```

#### Description
Individual highlighting for each part of a URL

| Component        | Example        |
|------------------|----------------|
| scheme           | `http`/`https` |
| host             | `google.com`   |
| path             | `/search`      |
| query parameters | `/?key=value`  |


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
