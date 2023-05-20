package highlighter

import "strings"

const (
	Normal = iota
	Black
	Red
	Green
	Yellow
	Blue
	Magenta
	Cyan
	White

	Reset     = "\033[0m"
	Bold      = "\033[1m"
	Faint     = "\033[2m"
	Italic    = "\033[3m"
	Underline = "\033[4m"
	Reverse   = "\033[7m"
	FgBlack   = "\033[30m"
	FgRed     = "\033[31m"
	FgGreen   = "\033[32m"
	FgYellow  = "\033[33m"
	FgBlue    = "\033[34m"
	FgMagenta = "\033[35m"
	FgCyan    = "\033[36m"
	FgWhite   = "\033[37m"
)

func Color(color, text string) string {
	return getColor(color) + text + Reset
}

func ColorStyle(color, styles, text string) string {
	return getStyles(styles) + getColor(color) + text + Reset
}

func ColorAndResetTo(color, text, resetToColor string) string {
	return getColor(color) + text + Reset + getColor(resetToColor)
}

func ColorStyleAndResetTo(color, styles, text, resetToColor, resetToStyles string) string {
	return getStyles(styles) + getColor(color) + text + Reset + getColor(resetToColor) + getStyles(resetToStyles)
}

func getColor(color string) string {
	switch color {
	case "black":
		return FgBlack
	case "red":
		return FgRed
	case "green":
		return FgGreen
	case "yellow":
		return FgYellow
	case "blue":
		return FgBlue
	case "magenta":
		return FgMagenta
	case "cyan":
		return FgCyan
	case "white":
		return FgWhite
	default:
		return ""
	}
}

func getStyles(style string) string {
	styles := ""

	for _, s2 := range strings.Fields(style) {
		styles += getStyle(s2)
	}

	return styles
}

func getStyle(style string) string {
	switch style {
	case "reset":
		return Reset
	case "bold":
		return Bold
	case "italic":
		return Italic
	case "faint":
		return Faint
	default:
		return ""
	}
}
