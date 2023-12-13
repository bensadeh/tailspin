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
- 🌈 Highlights numbers, dates, IP-addresses, UUIDs, URLs and more
- ⚙️ All highlight groups are customizable
- 🧬 Easy to integrate with other commands
- 🔍 Uses [`minus`](https://github.com/arijit79/minus) under the hood for scrollback, search and filtering

#

### Table of Contents

* [Overview](#overview)
* [Installing](#installing)
* [Highlight Groups](#highlight-groups)
* [Watching folders](#watching-folders)
* [Customizing Highlight Groups](#customizing-highlight-groups)
* [Working with `stdin` and `stdout`](#working-with-stdin-and-stdout)
* [Using the pager `minus`](#using-the-pager-minus)
* [Settings](#settings)

***

## Overview

`tailspin` works by reading through a log file line by line, running a series of regexes
against each line. The regexes recognize patterns you expect to find in a logfile, like dates, numbers, severity
keywords and more.

`tailspin` does not make any assumptions on the format or position of the items it wants to highlight. For this reason,
it requires no configuration and the highlighting will work consistently across different logfiles.

## Installing

### Package Managers

The binary name for `tailspin` is `tspin`.

```console
# Homebrew
brew install tailspin

# Cargo
cargo install tailspin

# Archlinux
pacman -S tailspin

# Nix
nix-shell -p tailspin

# NetBSD
pkgin install tailspin

# FreeBSD
pkg install tailspin
```

### From Source

```console
cargo install --path .
```

Binary will be placed in `~/.cargo/bin`, make sure you add the folder to your `PATH` environment variable.

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

### HTTP methods

<p align="center">
  <img src="assets/examples/http.png" width="600"/>
</p>

### UUIDs

<p align="center">
  <img src="assets/examples/uuids.png" width="600"/>
</p>

### Key-value pairs

<p align="center">
  <img src="assets/examples/kv.png" width="600"/>
</p>

### Unix processes

<p align="center">
  <img src="assets/examples/processes.png" width="600"/>
</p>

## Watching folders

`tailspin` can listen for newline entries in a given folder. Watching folders is useful for monitoring log files that
are rotated.

<p align="center">
  <img src="assets/examples/folder.png" width="600"/>
</p>

When watching folders, `tailspin` will start in follow mode (abort with <kbd>Ctrl + C</kbd>) and will only print
newline entries which arrive after the initial start.

## Customizing Highlight Groups

### Overview

Create `config.toml` in `~/.config/tailspin` to customize highlight groups.

Styles have the following shape:

```toml
style = { fg = "color", bg = "color", italic = false, bold = false, underline = false }
```

To edit the different highlight groups, include them in your `config.toml` file. For example, to edit the `date`
highlight group, add the following to your `config.toml`:

```toml
[date]
style = { fg = "green" }
```

Collapse the following section to see the default config:

<details>
<summary>Default highlight groups settings</summary>

```toml
[date]
style = { fg = "magenta" }
# To shorten the date, uncomment the line below
# shorten = { to = "␣", style = { fg = "magenta" } }

[time]
time = { fg = "blue" }
zone = { fg = "red" }
# To shorten the time, uncomment the line below
# shorten = { to = "␣", style = { fg = "blue" } }

[[keywords]]
words = ['null', 'true', 'false']
style = { fg = "red", italic = true }

[[keywords]]
words = ['GET']
style = { fg = "black", bg = "green" }
border = true

# You can add as many keywords as you'd like

[url]
http = { faint = true }
https = { bold = true }
host = { fg = "blue", faint = true }
path = { fg = "blue" }
query_params_key = { fg = "magenta" }
query_params_value = { fg = "cyan" }
symbols = { fg = "red" }

[number]
style = { fg = "cyan" }

[ip]
segment = { fg = "blue", italic = true }
separator = { fg = "red" }

[quotes]
style = { fg = "yellow" }
token = '"'

[path]
segment = { fg = "green", italic = true }
separator = { fg = "yellow" }

[uuid]
segment = { fg = "blue", italic = true }
separator = { fg = "red" }

[key_value]
key = { faint = true }
separator = { fg = "white" }

[process]
name = { fg = "green" }
separator = { fg = "red" }
id = { fg = "yellow" }
```

</details>

### Disabling Highlight Groups

To disable a highlight group, set the `disabled` field to true:

```toml
[date]
disabled = true
```

### Adding Keywords via config.toml

To add custom keywords, either include them in the list of keywords or add new entries:

```toml
[[keywords]]
words = ['MyCustomKeyword']
style = { fg = "green" }

[[keywords]]
words = ['null', 'true', 'false']
style = { fg = "red", italic = true }
```

### Adding Keywords from the command line

Sometimes it is more convenient to add highlight groups on the fly without having to edit a TOML. To add highlights from
the command line, use the `--words-[red|green|yellow|blue|magenta|cyan]` flag followed by a comma separated list
of words to be highlighted.

<p align="center">
  <img src="assets/examples/otf.png" width="800"/>
</p>

## Working with `stdin` and `stdout`

By default, `tailspin` will open a file in the pager `minus`. However, if you pipe something into `tailspin`, it will
print the highlighted output directly to `stdout`. This is similar to running `tspin [file] --print`.

To let `tailspin` highlight the logs of different commands, you can pipe the output of those commands into `tailspin`
like so:

```console
journalctl -f | tspin
cat /var/log/syslog | tspin
kubectl logs -f pod_name | tspin
```

## Using the pager `minus`

### Overview

`tailspin` uses `minus` as its pager to view the highlighted log files.
You can see the available keyibdings [here](https://docs.rs/minus/latest/minus/index.html#standard-actions).

## Settings

```console
-f, --follow                     Follow the contents of the file
-t, --tail                       Start at the end of the file
-p, --print                      Print the output to stdout
-c, --config-path PATH           Path to a custom configuration file
-l, --follow-command [CMD]       Follows the output of the provided command
    --words-red      [WORDS]     Highlight the provided words in red
    --words-green    [WORDS]     Highlight the provided words in green
    --words-yellow   [WORDS]     Highlight the provided words in yellow
    --words-blue     [WORDS]     Highlight the provided words in blue
    --words-magenta  [WORDS]     Highlight the provided words in magenta
    --words-cyan     [WORDS]     Highlight the provided words in cyan
    --disable-builtin-keywords   Disables the highlighting of all builtin keyword groups
    --disable-booleans           Disables the highlighting of booleans and nulls
    --disable-severity           Disables the highlighting of severity levels
    --disable-rest               Disables the highlighting of REST verbs
```



