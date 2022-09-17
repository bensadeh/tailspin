package syntax

import (
	. "github.com/logrusorgru/aurora/v3"
	"regexp"
	"spin/core"
	"spin/highlighter"
	"spin/parser"
	"spin/replace"
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
		input = strings.ReplaceAll(input, keyword.String, highlighter.Color(keyword.Fg, keyword.String))
	}

	return input
}

func highlightWithRegExp(input string, regExpressions []*core.RegularExpression) string {
	for _, regExpression := range regExpressions {
		expression := regexp.MustCompile(regExpression.RegExp)

		input = expression.ReplaceAllString(input, highlighter.Color(regExpression.Fg, `$0`))
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
	start := `[CLX_URL_START]`
	stop := `[CLX_URL_STOP]`

	expression := regexp.MustCompile(
		`(?P<protocol>http[s]?:)?//(?P<host>[a-z0-9A-Z-_.]+)(?P<port>:\d+)?(?P<path>[\/a-zA-Z0-9-\.]+)?(?P<search>\?[^#\n]+)?`)
	input = expression.ReplaceAllString(input, start+
		Green(`$protocol`).String()+"//"+Blue(`$host`).String()+Cyan(`$port`).String()+
		Yellow(`$path`).String()+`$search`+stop)

	//input = replace.SearchAndReplaceInBetweenTokens("?", stop, input, "?", color.ColorAndResetTo("red", "?", "green"))
	input = replace.SearchAndReplaceInBetweenTokens("?", stop, input, "&", highlighter.ColorAndResetTo("red", "&", "cyan"))
	input = replace.SearchAndReplaceInBetweenTokens("?", stop, input, "=", highlighter.ColorAndResetTo("red", "=", "magenta"))
	input = replace.SearchAndReplaceInBetweenTokens(start, stop, input, "?", highlighter.ColorAndResetTo("red", "?", "cyan"))

	//questionMarks := regexp.MustCompile(`(` + start + `.*)(\?)(.*` + stop + `)`)
	//input = questionMarks.ReplaceAllString(input, `$1`+Red(`$2`).String()+`$3`)
	//
	//ampersands := regexp.MustCompile(`(` + start + `.*)(\&)(.*` + stop + `)`)
	//input = ampersands.ReplaceAllString(input, `$1`+Red(`$2`).String()+`$3`)
	//
	//equals := regexp.MustCompile(`(` + start + `.*)(\=)(.*` + stop + `)`)
	//input = equals.ReplaceAllString(input, `$1`+Red(`$2`).String()+`$3`)

	input = strings.ReplaceAll(input, start, "")
	input = strings.ReplaceAll(input, stop, "")

	return input
}

func highlightJavaExceptionHeader(input string) string {
	expression := regexp.MustCompile(`(?:([^:,\s]+):\s+([^:\n]+)|(!\A))`)

	return expression.ReplaceAllString(input, Yellow(`$1`).String()+Red(`$2`).String()+"("+Magenta(`$3`).String()+":"+Cyan(`$4`).String()+")")
}

func highlightJavaExceptionBody(input string) string {
	start := "[JAVA_EXCEPTION_BODY_START]"
	stop := "[JAVA_EXCEPTION_BODY_STOP]"

	expression := regexp.MustCompile(`(^\s*at)(\s+\S+)\((\w+\.\w+|Unknown Source)(:?)(\d+)?\)`)

	input = expression.ReplaceAllString(input, start+Yellow(`$1`).String()+Red(`$2`).String()+"("+
		Magenta(`$3`).String()+`$4`+Cyan(`$5`).String()+")"+stop)

	input = replace.SearchAndReplaceInBetweenTokens(start, stop, input, "Unknown Source",
		highlighter.ColorStyle("", "reset faint", "Unknown Source"))

	input = replace.SearchAndReplaceInBetweenTokens(start, stop, input, ".java",
		highlighter.ColorStyle("", "reset", ".java"))

	input = strings.ReplaceAll(input, start, "")
	input = strings.ReplaceAll(input, stop, "")

	return input
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
