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
		text = highlightDate(text)
		text = highlightUrl(text)
		text = highlightWithRegExp(text, scheme.RegularExpressions)
		text = highlightJavaExceptionHeader(text)
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
		if keyword.Strict {
			input = strings.ReplaceAll(input, keyword.String, highlighter.ColorStyle(keyword.Fg, keyword.Style, keyword.String))

			continue
		}

		lineHasKeywordOnly := regexp.MustCompile(`^` + keyword.String + `$`)
		input = lineHasKeywordOnly.ReplaceAllString(input, highlighter.ColorStyle(keyword.Fg, keyword.Style, `$0`))

		expression := regexp.MustCompile(`([ |[|(]|=)(` + keyword.String + `)([]|:| |,|.|)])`)
		input = expression.ReplaceAllString(input, `$1`+highlighter.ColorStyle(keyword.Fg, keyword.Style, `$2`)+`$3`)
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

func highlightDate(input string) string {
	dayMonthYear := regexp.MustCompile(`(Mon|Tue|Wed|Thu|Fri|Sat|Sun)? ?(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec) \d{1,2}( ?\d{4})?`)
	input = dayMonthYear.ReplaceAllString(input, Yellow(`$0`).String())

	date := regexp.MustCompile(`20\d{2}(.\d{2}){2}`)
	input = date.ReplaceAllString(input, Yellow(`$0`).String())

	time := regexp.MustCompile(`(\s|T)(\d{2}.){2}\d{2}[ |,|\.|+]\d{3,6}`)
	input = time.ReplaceAllString(input, Yellow(`$0`).String())

	simpleTime := regexp.MustCompile(` \d{2}:\d{2}:\d{2} `)
	input = simpleTime.ReplaceAllString(input, Yellow(`$0`).String())

	return input
}

func highlightUrl(input string) string {
	start := `[URL_START]`
	stop := `[URL_STOP]`

	expression := regexp.MustCompile(
		`(?P<protocol>http[s]?:)?//(?P<host>[a-z0-9A-Z-_.]+)(?P<port>:\d+)?(?P<path>[\/a-zA-Z0-9-\.]+)?(?P<search>\?[^#\n]+)?`)
	input = expression.ReplaceAllString(input, start+
		Green(`$protocol`).String()+"//"+Blue(`$host`).String()+Cyan(`$port`).String()+
		Yellow(`$path`).String()+`$search`+stop)

	input = replace.SearchAndReplaceInBetweenTokens("?", stop, input, "&", highlighter.ColorAndResetTo("red", "&", "cyan"))
	input = replace.SearchAndReplaceInBetweenTokens("?", stop, input, "=", highlighter.ColorAndResetTo("red", "=", "magenta"))
	input = replace.SearchAndReplaceInBetweenTokens(start, stop, input, "?", highlighter.ColorAndResetTo("red", "?", "cyan"))

	input = strings.ReplaceAll(input, start, "")
	input = strings.ReplaceAll(input, stop, "")

	return input
}

func highlightJavaExceptionHeader(input string) string {
	expression := regexp.MustCompile(`(\S+\.)([A-Z]\S+Exception)(: )`)

	return expression.ReplaceAllString(input, Red(`$1`).String()+Red(`$2`).Bold().String()+`$3`)
}

func highlightJavaExceptionBody(input string) string {
	start := "[JAVA_EXCEPTION_BODY_START]"
	stop := "[JAVA_EXCEPTION_BODY_STOP]"

	expression := regexp.MustCompile(`(^\s*at)(\s+\S+)\((\w+\.\w+|Unknown Source|Native Method)(:?)(\d+)?\)`)

	input = expression.ReplaceAllString(input, start+Yellow(`$1`).String()+Red(`$2`).String()+"("+
		Magenta(`$3`).String()+`$4`+Cyan(`$5`).String()+")"+stop)

	input = replace.SearchAndReplaceInBetweenTokens(start, stop, input, "Unknown Source",
		highlighter.ColorStyle("", "reset faint", "Unknown Source"))

	input = replace.SearchAndReplaceInBetweenTokens(start, stop, input, "Native Method",
		highlighter.ColorStyle("green", "reset faint", "Native Method"))

	input = replace.SearchAndReplaceInBetweenTokens(start, stop, input, ".java",
		highlighter.ColorStyle("", "reset", ".java"))

	input = strings.ReplaceAll(input, start, "")
	input = strings.ReplaceAll(input, stop, "")

	return input
}

func highlightDigits(input string) string {
	hasKeywordOnly := regexp.MustCompile(`^\d+$`)
	input = hasKeywordOnly.ReplaceAllString(input, Cyan(`$0`).String())

	expression := regexp.MustCompile(`([ |\[|(])(\d+\.)*(\d+)([\s|$|,|\]|)])`)

	return expression.ReplaceAllString(input, `$1`+Cyan(`$2`).String()+Cyan(`$3`).String()+`$4`)
}

func highlightGUIDs(input string) string {
	expression := regexp.MustCompile(`\b([a-zA-Z 0-9]{8})-([a-zA-Z 0-9]{4})-([a-zA-Z 0-9]{4})-([a-zA-Z 0-9]{4})-([a-zA-Z 0-9]{12})[^/]`)

	return expression.ReplaceAllString(input, Yellow(`$1`).Italic().String()+
		Red("-").String()+Yellow(`$2`).Italic().String()+
		Red("-").String()+Yellow(`$3`).Italic().String()+
		Red("-").String()+Yellow(`$4`).Italic().String()+
		Red("-").String()+Yellow(`$5`).Italic().String())
}

func highlightConstants(input string) string {
	expression := regexp.MustCompile(`[A-Z\d_]+_[A-Z\d]+[^a-z]\b`)

	return expression.ReplaceAllString(input, Magenta(`$0`).Italic().String())
}
