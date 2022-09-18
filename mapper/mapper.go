package mapper

import (
	"spin/core"
	"spin/styling"
)

func MapTheme(theme *styling.Theme) *core.Scheme {
	scheme := core.Scheme{}

	scheme.Keywords = FlattenKeywords(theme.Keywords)
	scheme.RegularExpressions = FlattenRegularExpressions(theme.RegularExpressions)

	return &scheme
}

func FlattenKeywords(keywords []styling.Keyword) []*core.Keyword {
	var flatKeywords []*core.Keyword

	for _, item := range keywords {
		for _, str := range item.Strings {

			var sAndR = core.Keyword{
				String: str,
				Fg:     item.Fg,
				Style:  item.Style,
				Strict: item.Strict,
			}
			flatKeywords = append(flatKeywords, &sAndR)
		}
	}

	return flatKeywords
}

func FlattenRegularExpressions(regExpressions []styling.RegularExpression) []*core.RegularExpression {
	var flatRegExpressions []*core.RegularExpression

	for _, item := range regExpressions {
		for _, regexp := range item.Regexp {

			var regExpressions = core.RegularExpression{
				RegExp: regexp,
				Fg:     item.Fg,
			}
			flatRegExpressions = append(flatRegExpressions, &regExpressions)
		}
	}

	return flatRegExpressions
}
