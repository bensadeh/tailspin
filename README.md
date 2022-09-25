<p align="center">
  <img src="assets/logo-new.png" width="600"/>
</p>

<p align="center">
A log file highlighter
</p>

<p align="center">
  <img src="assets/example.png" width="700"/>
</p>

## Overview

`tailspin` is a command line tool for viewing (and `tail`-ing) log files. It highlights important keywords to make
navigating log files easier.

## See also

* [lnav](https://github.com/tstack/lnav)
* [grc](https://github.com/garabik/grc)

## Under the hood

`tailspin` uses:

* [`less`](http://greenwoodsoftware.com/less/)
* [Bubble Tea](https://github.com/charmbracelet/bubbletea) for handling control over to `less`
* [cobra](https://github.com/spf13/cobra) for the CLI