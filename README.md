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
cargo install --path . 

# View log file
spin [file]

# Tail log file
spin [file] -f
```

## Highlight Groups

### Overview

`tailspin` uses a single `config.toml` file to configure all highlight groups. When customizing highlights it is advised
to start with the tailspin generate_config command to place a `config.toml` with default options
in `~/.config/tailspin`.

To disable a highlight group, either comment it out or delete it.

Highlights have the following shape:

```toml
style = { fg = "color", bg = "color", italic = false, bold = false, underline = false }
```

### Dates

```toml
[groups.date]
style = { fg = "magenta" }
```

Matches any date in the following formats:

- `YYYY-MM-DD`
- `YYYY-MM-DD HH:MM:SS`
- `YYYY-MM-DD HH:MM:SS,SSS`

### Keywords

```toml
[[groups.keywords]]
words = ['DEBUG']
style = { fg = "green" }

[[groups.keywords]]
words = ['null', 'true', 'false']
style = { fg = "red", italic = true }
```

The `keywords` group is used to highlight strings. Keywords are highlighted if they are within a `\b` regexp word
boundary. For example:

- It would match `cat` in the sentence "The **cat** is cute." because there are word boundaries
  (spaces in this case) around `cat`.
- It would match `cat` in the sentence "Is that a **cat**?" because there are word boundaries
  (space and question mark) around `cat`.
- It wouldn't match "cat" in the word "concatenate" because `cat` is bounded by other word
  characters (letters in this case) and not by word boundaries.

### URLs

```toml
[groups.url]
http = { faint = true }
https = { bold = true }
host = { fg = "blue", faint = true }
path = { fg = "blue" }
query_params_key = { fg = "cyan" }
query_params_value = { fg = "magenta" }
symbols = { fg = "red" }
```

Highlights the different segments of a URL.

### Numbers

```toml
[groups.number]
style = { fg = "cyan" }
```

Highlights any number (integer or float).

### IP

```toml
[groups.ip]
segment = { fg = "blue", italic = true }
separator = { fg = "red" }
```

Highlights IPv4 addresses in the following format:

- `10.0.0.1`
- `192.168.0.1`

### Quotes

```toml
[groups.quotes]
style = { fg = "yellow" }
token = '"'
```

Highlights any string that is wrapped in quotes.

### Unix file paths

```toml
[groups.path]
segment = { fg = "green", italic = true }
separator = { fg = "yellow" }
```

Highlights Unix file paths in the following format:

- `/etc/var/`
- `/path/to/file.txt`

### UUIDs

```toml
[groups.uuid]
segment = { fg = "blue", italic = true }
separator = { fg = "red" }
```

Highlights UUIDs in the following format:

- `123e4567-e89b-12d3-a456-426614174000`

## Settings

```console
-f, --follow       Follow (tail) the contents of the file
    --config       Provide a custom path configuration file
```



