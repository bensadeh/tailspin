package syntax

import (
	. "github.com/logrusorgru/aurora/v3"
	"regexp"
	"strings"
)

const (
	reset = "\033[0m"
)

func Highlight(line string) string {
	line = highlightCommonKeywords(line)
	line = highlightTime(line)
	line = highlightDate(line)

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

func highlightTime(input string) string {
	expression := regexp.MustCompile(`\d{2}:\d{2}:\d{2}\.\d{2,3}`)

	return expression.ReplaceAllString(input, Magenta(`$0`).String())
}

func highlightDate(input string) string {
	expression := regexp.MustCompile(`\d{4}-\d{2}-\d{2}`)

	return expression.ReplaceAllString(input, Cyan(`$0`).String())
}
