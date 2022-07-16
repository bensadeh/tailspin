package syntax

import (
	. "github.com/logrusorgru/aurora/v3"
	"strings"
)

const (
	reset = "\033[0m"
)

func Highlight(line string) string {
	line = highlightCommonKeywords(line)

	return reset + line
}

func highlightCommonKeywords(input string) string {
	input = strings.ReplaceAll(input, "ERROR", Red("ERROR").String())
	input = strings.ReplaceAll(input, "error", Red("error").String())

	input = strings.ReplaceAll(input, "INFO", Blue("INFO").String())
	input = strings.ReplaceAll(input, "DEBUG", Yellow("DEBUG").String())
	input = strings.ReplaceAll(input, "TRACE", Faint("TRACE").Italic().String())

	return input
}
