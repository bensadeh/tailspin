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
		resetToColor := getQuotesColor(segment.Separator)

		text := segment.Content

		text = highlightKeywords(text, scheme.Keywords, resetToColor)
		text = highlightDate(text, resetToColor)
		text = highlightUrl(text, resetToColor)
		text = highlightWithRegExp(text, scheme.RegularExpressions, resetToColor)
		text = highlightJavaExceptionHeader(text)
		text = highlightJavaExceptionBody(text)

		text = highlightGUIDs(text, resetToColor)
		text = highlightNumbers(text, "cyan", resetToColor)
		//text = highlightConstants(text)

		separator := Green(segment.Separator).String()

		if segment.Separator == `"` {
			separator = separator + green
		}

		highlightedLine = highlightedLine + separator + text + separator + reset
	}

	return reset + highlightedLine
}

func getQuotesColor(separator string) (color string) {
	if separator == `"` {
		return "green"
	}

	return ""
}

func highlightKeywords(input string, keywords []*core.Keyword, resetToColor string) string {
	for _, keyword := range keywords {
		if keyword.Strict {
			input = strings.ReplaceAll(input, keyword.String, highlighter.ColorStyleAndResetTo(keyword.Fg,
				keyword.Style, keyword.String, resetToColor, ""))

			continue
		}

		lineHasKeywordOnly := regexp.MustCompile(`^` + keyword.String + `$`)
		input = lineHasKeywordOnly.ReplaceAllString(input, highlighter.ColorStyleAndResetTo(keyword.Fg,
			keyword.Style, `$0`, resetToColor, ""))

		expression := regexp.MustCompile(`([ |[|(]|=)(` + keyword.String + `)([]|:| |,|.|)])`)
		input = expression.ReplaceAllString(input, `$1`+highlighter.ColorStyleAndResetTo(keyword.Fg,
			keyword.Style, `$2`, resetToColor, "")+`$3`)
	}

	return input
}

func highlightWithRegExp(input string, regExpressions []*core.RegularExpression, resetToColor string) string {
	for _, regExpression := range regExpressions {
		expression := regexp.MustCompile(regExpression.RegExp)

		input = expression.ReplaceAllString(input, highlighter.ColorAndResetTo(regExpression.Fg, `$0`, resetToColor))
	}

	return input
}

func highlightDate(input string, resetToColor string) string {
	dayMonthYear := regexp.MustCompile(
		`(Mon|Tue|Wed|Thu|Fri|Sat|Sun)? ?(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec) \d{1,2}( ?\d{4})?`)
	input = dayMonthYear.ReplaceAllString(input, highlighter.ColorStyleAndResetTo("yellow", "", `$0`,
		resetToColor, ""))

	date := regexp.MustCompile(`20\d{2}(.\d{2}){2}`)
	input = date.ReplaceAllString(input, highlighter.ColorStyleAndResetTo("yellow", "", `$0`,
		resetToColor, ""))

	time := regexp.MustCompile(`(\s|T)(\d{2}.){2}\d{2}[ |,|\.|+]\d{3,6}`)
	input = time.ReplaceAllString(input, highlighter.ColorStyleAndResetTo("yellow", "", `$0`,
		resetToColor, ""))

	simpleTime := regexp.MustCompile(` \d{2}:\d{2}:\d{2} `)
	input = simpleTime.ReplaceAllString(input, highlighter.ColorStyleAndResetTo("yellow", "", `$0`,
		resetToColor, ""))

	return input
}

func highlightUrl(input string, resetToColor string) string {
	start := `[URL_START]`
	stop := `[URL_STOP]`

	expression := regexp.MustCompile(
		`(?P<protocol>http[s]?:)?//(?P<host>[a-z0-9A-Z-_.]+)(?P<port>:\d+)?(?P<path>[\/a-zA-Z0-9-\.]+)?(?P<search>\?[^#\n ]+)?`)
	input = expression.ReplaceAllString(input, start+
		Green(`$protocol`).String()+"//"+Blue(`$host`).String()+Cyan(`$port`).String()+
		Yellow(`$path`).String()+`$search`+stop)

	input = replace.SearchAndReplaceInBetweenTokens("?", stop, input, "&", highlighter.ColorAndResetTo("red", "&", "cyan"))
	input = replace.SearchAndReplaceInBetweenTokens("?", stop, input, "=", highlighter.ColorAndResetTo("red", "=", "magenta"))
	input = replace.SearchAndReplaceInBetweenTokens(start, stop, input, "?", highlighter.ColorAndResetTo("red", "?", "cyan"))

	input = strings.ReplaceAll(input, start, "")
	input = strings.ReplaceAll(input, stop, highlighter.ColorStyleAndResetTo("", "", "", resetToColor, ""))

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

func highlightNumbers(input string, color string, resetToColor string) string {
	hasKeywordOnly := regexp.MustCompile(`^\d+$`)
	input = hasKeywordOnly.ReplaceAllString(input, highlighter.ColorAndResetTo(color, `$0`, resetToColor))

	// We handle this special case to avoid highlighting numbers in timestamps, i.e. 10:00
	noTime := regexp.MustCompile(`([\D]:)(\d+)`)
	input = noTime.ReplaceAllString(input, "$1"+highlighter.ColorAndResetTo(color, `$2`, resetToColor))

	mostCommonNumberMatches := regexp.MustCompile(`([ \[|(=])(\d+\.)*(\d+)([\s,\])])`)
	input = mostCommonNumberMatches.ReplaceAllString(input, `$1`+highlighter.Color(color, `$2`)+
		highlighter.ColorAndResetTo(color, `$3`, resetToColor)+`$4`)

	return input
}

func highlightGUIDs(input string, resetToColor string) string {
	expression := regexp.MustCompile(
		`\b([a-zA-Z 0-9]{8})-([a-zA-Z 0-9]{4})-([a-zA-Z 0-9]{4})-([a-zA-Z 0-9]{4})-([a-zA-Z 0-9]{12})[^/]`)

	return expression.ReplaceAllString(input, Yellow(`$1`).Italic().String()+
		Red("-").String()+Yellow(`$2`).Italic().String()+
		Red("-").String()+Yellow(`$3`).Italic().String()+
		Red("-").String()+Yellow(`$4`).Italic().String()+
		Red("-").String()+
		highlighter.ColorStyleAndResetTo("yellow", "italic", `$5`, resetToColor, ""))
}

func highlightConstants(input string, resetToColor string) string {
	expression := regexp.MustCompile(`[A-Z\d_]+_[A-Z\d]+[^a-z]\b`)
	input = expression.ReplaceAllString(input, highlighter.ColorStyleAndResetTo("magenta", "italic", `$0`,
		resetToColor, ""))

	return input
}
