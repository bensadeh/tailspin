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
- 🔍 Uses `less` under the hood for scrollback, search and filtering

#

### Table of Contents

* [Overview](#overview)
* [Installing](#installing)
* [Highlight Groups](#highlight-groups)
* [Watching folders](#watching-folders)
* [Customizing Highlight Groups](#customizing-highlight-groups)
* [Working with `stdin` and `stdout`](#working-with-stdin-and-stdout)
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

The binary name for `tailspin` is `spin`.

```console
# Cargo
cargo install tailspin

# Debian
apt install tailspin

# AUR
paru -S tailspin

# Nix
nix-shell -p tailspin
```

## Highlight Groups

### Dates

<p align="center">
  <img src="assets/examples/dates.png" width="600"/>
</p>

<details>
<summary>Config</summary>

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
```

</details>

### Keywords

<p align="center">
  <img src="assets/examples/keywords.png" width="600"/>
</p>

<details>
<summary>Config</summary>

```toml
[[keywords]]
words = ['null', 'true', 'false']
style = { fg = "red", italic = true }

[[keywords]]
words = ['GET']
style = { fg = "black", bg = "green" }
border = true

# You can add as many keywords as you'd like
```

</details>

### URLs

<p align="center">
  <img src="assets/examples/urls.png" width="600"/>
</p>

<details>
<summary>Config</summary>

```toml
[url]
http = { faint = true }
https = { bold = true }
host = { fg = "blue", faint = true }
path = { fg = "blue" }
query_params_key = { fg = "magenta" }
query_params_value = { fg = "cyan" }
symbols = { fg = "red" }
```

</details>

### Numbers

<p align="center">
  <img src="assets/examples/numbers.png" width="600"/>
</p>

<details>
<summary>Config</summary>

```toml
[number]
style = { fg = "cyan" }
```

</details>

### IP Addresses

<p align="center">
  <img src="assets/examples/ip.png" width="600"/>
</p>

<details>
<summary>Config</summary>

```toml
[ip]
segment = { fg = "blue", italic = true }
separator = { fg = "red" }
```

</details>

### Quotes

<p align="center">
  <img src="assets/examples/quotes.png" width="600"/>
</p>

<details>
<summary>Config</summary>

```toml
[quotes]
style = { fg = "yellow" }
token = '"'
```

</details>

### Unix file paths

<p align="center">
  <img src="assets/examples/paths.png" width="600"/>
</p>

<details>
<summary>Config</summary>

```toml
[path]
segment = { fg = "green", italic = true }
separator = { fg = "yellow" }
```

</details>

### HTTP methods

<p align="center">
  <img src="assets/examples/http.png" width="600"/>
</p>

<details>
<summary>Config</summary>
See Keywords
</details>

### UUIDs

<p align="center">
  <img src="assets/examples/uuids.png" width="600"/>
</p>

<details>
<summary>Config</summary>

```toml
[uuid]
segment = { fg = "blue", italic = true }
separator = { fg = "red" }
```

</details>

### Key-value pairs

<p align="center">
  <img src="assets/examples/kv.png" width="600"/>
</p>

<details>
<summary>Config</summary>

```toml
[key_value]
key = { faint = true }
separator = { fg = "white" }
```

</details>

### Unix processes

<p align="center">
  <img src="assets/examples/processes.png" width="600"/>
</p>

<details>
<summary>Config</summary>

```toml
[process]
name = { fg = "green" }
separator = { fg = "red" }
id = { fg = "yellow" }
```

</details>

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

### Disabling Highlight Groups

To disable a highlight group, set the `disabled` field to true:

```toml
[date]
disabled = true
```

### Adding Keywords

To add custom keywords, either include them in the list of keywords or add new entries:

```toml
[[keywords]]
words = ['MyCustomKeyword']
style = { fg = "green" }

[[keywords]]
words = ['null', 'true', 'false']
style = { fg = "red", italic = true }
```

## Working with `stdin` and `stdout`

By default, `tailspin` will open a file in the pager `less`. However, if you pipe something into `tailspin`, it will
print the highlighted output directly to `stdout`. This is similar to running `spin [file] --print`.

To let `tailspin` highlight the logs of different commands, you can pipe the output of those commands into `tailspin`
like so:

```console
journalctl -f | spin
cat /var/log/syslog | spin
kubectl logs -f pod_name | spin
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

To resume following the file from within `less`, press <kbd>Shift + F</kbd>.

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
-f, --follow                 Follow the contents of the file
-t, --tail                   Start at the end of the file
-p, --print                  Print the output to stdout
-c, --config-path PATH       Path to a custom configuration file
-t, --follow-command 'CMD'   Follows the output of the provided command
    --create-default-config  Generate a new configuration file
    --show-default-config    Print the default configuration
```



