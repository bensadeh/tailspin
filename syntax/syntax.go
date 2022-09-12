package syntax

import (
	. "github.com/logrusorgru/aurora/v3"
	"regexp"
	"spin/color"
	"spin/core"
	"spin/parser"
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

func Highlight(line string, scheme *core.Scheme) string {
	// Carriage return (\r) messes with the regexp, so we remove it
	line = strings.ReplaceAll(line, "\r", "")
	line = line + " "

	highlightedLine := ""

	segments := parser.ExtractSegments(line)

	for _, segment := range segments {
		text := segment.Content

		text = highlightCommonKeywords(text, scheme.Keywords)
		text = highlightTime(text)
		text = highlightDateInDigits(text)
		text = highlightUrl(text)
		text = highlightWithRegExp(text, scheme.RegularExpressions)
		//text = highlightJavaExceptionHeader(text)
		text = highlightJavaExceptionBody(text)

		text = highlightGUIDs(text)
		text = highlightDigits(text)
		text = highlightConstants(text)

		separator := Green(segment.Separator).String()

		if segment.Separator == `"` {
			separator = separator + green
		}

		highlightedLine = highlightedLine + separator + text + separator + reset
	}

	return reset + highlightedLine
}

func highlightCommonKeywords(input string, keywords []*core.Keyword) string {
	for _, keyword := range keywords {
		input = strings.ReplaceAll(input, keyword.String, color.C(keyword.Fg, keyword.String))
	}

	return input
}

func highlightWithRegExp(input string, regExpressions []*core.RegularExpression) string {
	for _, regExpression := range regExpressions {
		expression := regexp.MustCompile(regExpression.RegExp)

		input = expression.ReplaceAllString(input, color.C(regExpression.Fg, `$0`))
	}

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

func highlightUrl(input string) string {
	expression := regexp.MustCompile(
		`(?P<protocol>http[s]?:)?//(?P<host>[a-z0-9A-Z-_.]+)(?P<port>:\d+)?(?P<path>[\/a-zA-Z0-9-\.]+)?(?P<search>\?[^#\n]+)?`)
	return expression.ReplaceAllString(input,
		Yellow(`$protocol`).String()+"//"+Blue(`$host`).String()+Cyan(`$port`).String()+
			Red(`$path`).String()+Green(`$search`).String())
}

func highlightJavaExceptionHeader(input string) string {
	expression := regexp.MustCompile(`(?:([^:,\s]+):\s+([^:\n]+)|(!\A))`)

	return expression.ReplaceAllString(input, Yellow(`$1`).String()+Red(`$2`).String()+"("+Magenta(`$3`).String()+":"+Cyan(`$4`).String()+")")
}

func highlightJavaExceptionBody(input string) string {
	expression := regexp.MustCompile(`(^\s*at)(\s+\S+)\((\w+\.\w+|Unknown Source)(:?)(\d+)?\)`)

	return expression.ReplaceAllString(input, Yellow(`$1`).String()+Red(`$2`).String()+"("+Magenta(`$3`).String()+`$4`+Cyan(`$5`).String()+")")
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
