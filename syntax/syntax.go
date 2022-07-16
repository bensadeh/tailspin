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
	line = highlightDateInDigits(line)
	line = highlightDateInWords(line)

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

func highlightDateInDigits(input string) string {
	expression := regexp.MustCompile(`\d{4}-\d{2}-\d{2}`)

	return expression.ReplaceAllString(input, Cyan(`$0`).String())
}

func highlightDateInWords(input string) string {
	expression := regexp.MustCompile(`(Mon|Tue|Wed|Thu|Fri|Sat|Sun) (Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec) \d{2}`)

	return expression.ReplaceAllString(input, Cyan(`$0`).String())
}
