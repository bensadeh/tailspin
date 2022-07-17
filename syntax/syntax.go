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
	// Carriage return (\r) messes with the regexp, so we remove it
	line = strings.ReplaceAll(line, "\r", "")
	line = line + " "

	line = highlightCommonKeywords(line)
	line = highlightTime(line)
	line = highlightDateInDigits(line)
	line = highlightDateInWords(line)
	line = highlightGUIDs(line)
	line = highlightDigits(line)
	line = highlightConstants(line)
	line = highlightExceptions(line)

	return reset + line
}

func highlightCommonKeywords(input string) string {
	input = strings.ReplaceAll(input, "null", Red("null").String())
	input = strings.ReplaceAll(input, "NULL", Red("NULL").String())
	input = strings.ReplaceAll(input, "nil", Red("nil").String())
	input = strings.ReplaceAll(input, "true", Red("true").String())
	input = strings.ReplaceAll(input, "false", Red("false").String())

	input = strings.ReplaceAll(input, "ERROR", Red("ERROR").String())
	input = strings.ReplaceAll(input, "FAIL", Red("FAIL").String())
	input = strings.ReplaceAll(input, "FAILURE", Red("FAILURE").String())
	input = strings.ReplaceAll(input, "error", Red("error").String())

	input = strings.ReplaceAll(input, "INFO", Blue("INFO").String())
	input = strings.ReplaceAll(input, "DEBUG", Green("DEBUG").String())
	input = strings.ReplaceAll(input, "WARN", Yellow("WARN").String())
	input = strings.ReplaceAll(input, "WARNING", Yellow("WARNING").String())
	input = strings.ReplaceAll(input, "TRACE", Faint("TRACE").Italic().String())

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
	expression := regexp.MustCompile(` \d+[\s|$]`)

	return expression.ReplaceAllString(input, Cyan(`$0`).String())
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
