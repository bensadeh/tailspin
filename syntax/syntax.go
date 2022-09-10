package syntax

import (
	. "github.com/logrusorgru/aurora/v3"
	"regexp"
	"spin/block"
	"strings"
)

const (
	reset   = "\033[0m"
	bold    = "\033[1m"
	reverse = "\033[7m"
	italic  = "\033[3m"
	magenta = "\033[35m"
	faint   = "\033[2m"
	green   = "\033[32m"
	red     = "\033[31m"
)

func Highlight(line string) string {
	// Carriage return (\r) messes with the regexp, so we remove it
	line = strings.ReplaceAll(line, "\r", "")
	line = line + " "

	highlightedLine := ""

	segments := block.ExtractSegments(line)

	for _, segment := range segments {
		text := segment.Content

		text = highlightCommonKeywords(text)
		text = highlightTime(text)
		text = highlightDateInDigits(text)
		text = highlightDateInWords(text)
		text = highlightGUIDs(text)
		text = highlightDigits(text)
		text = highlightConstants(text)
		text = highlightExceptions(text)

		separator := Green(segment.Separator).String()

		if segment.Separator == `"` {
			separator = separator + green
		}

		highlightedLine = highlightedLine + separator + text + separator + reset
	}

	return reset + highlightedLine
}

func highlightCommonKeywords(input string) string {
	input = strings.ReplaceAll(input, "null", Red("null").Italic().String())
	input = strings.ReplaceAll(input, "NULL", Red("NULL").Italic().String())
	input = strings.ReplaceAll(input, "nil", Red("nil").Italic().String())
	input = strings.ReplaceAll(input, "true", Red("true").Italic().String())
	input = strings.ReplaceAll(input, "false", Red("false").Italic().String())

	input = strings.ReplaceAll(input, "ERROR", Red("ERROR").String())
	input = strings.ReplaceAll(input, "FAIL", Red("FAIL").String())
	input = strings.ReplaceAll(input, "FAILURE", Red("FAILURE").String())
	input = strings.ReplaceAll(input, "error", Red("error").String())

	input = strings.ReplaceAll(input, "INFO", Blue("INFO").String())
	input = strings.ReplaceAll(input, "DEBUG", Green("DEBUG").String())
	input = strings.ReplaceAll(input, "WARN", Yellow("WARN").String())
	input = strings.ReplaceAll(input, "WARNING", Yellow("WARNING").String())
	input = strings.ReplaceAll(input, "TRACE", Faint("TRACE").Italic().String())

	input = strings.ReplaceAll(input, "GET", Green("GET").String())
	input = strings.ReplaceAll(input, "PUT", Yellow("PUT").String())
	input = strings.ReplaceAll(input, "POST", Magenta("POST").String())
	input = strings.ReplaceAll(input, "PATCH", Blue("PATCH").String())
	input = strings.ReplaceAll(input, "DELETE", Red("DELETE").String())

	return input
}

func highlightTime(input string) string {
	expression := regexp.MustCompile(`\d{2}:\d{2}:\d{2}(\.\d{2,3}| )`)

	return expression.ReplaceAllString(input, Magenta(`$0`).String())
}

func highlightDateInDigits(input string) string {
	expression := regexp.MustCompile(`\d{4}-\d{2}-\d{2}`)

	return expression.ReplaceAllString(input, Magenta(`$0`).String())
}

func highlightDateInWords(input string) string {
	expression := regexp.MustCompile(
		`(Mon|Tue|Wed|Thu|Fri|Sat|Sun) (Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec) \d{2}`)

	return expression.ReplaceAllString(input, Magenta(`$0`).String())
}

func highlightDigits(input string) string {
	expression := regexp.MustCompile(`([ |\[|(])(\d+)([\s|$|,|\]|)])`)

	return expression.ReplaceAllString(input, `$1`+Cyan(`$2`).String()+`$3`)
}

func highlightGUIDs(input string) string {
	expression := regexp.MustCompile(`[0-9a-fA-F]+-[0-9a-fA-F]+-[0-9a-fA-F]+-[0-9a-fA-F]+-[0-9a-fA-F]+`)

	return expression.ReplaceAllString(input, Yellow(`$0`).String())
}

func highlightConstants(input string) string {
	expression := regexp.MustCompile(`[A-Z\d]*_[A-Z\d_]+`)

	return expression.ReplaceAllString(input, Yellow(`$0`).Italic().String())
}

func highlightExceptions(input string) string {
	expression := regexp.MustCompile(`[\w|.]+Exception`)

	return expression.ReplaceAllString(input, Red(`$0`).Italic().String())
}
