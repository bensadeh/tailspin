package highlighter

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

func ColorStyle(color, style, text string) string {
	return getColor(color) + getStyle(style) + text + Reset
}

func ColorAndResetTo(color, text, resetToColor string) string {
	return getColor(color) + text + getColor(resetToColor)
}

func ColorStyleAndResetTo(color, style, text, resetToColor, resetToStyle string) string {
	return getColor(color) + getStyle(style) + text + getColor(resetToColor) + getStyle(resetToStyle)
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
