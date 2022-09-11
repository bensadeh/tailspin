package mapper

import (
	"spin/core"
	"spin/styling"
)

func MapTheme(theme *styling.Theme) *core.Scheme {
	scheme := core.Scheme{}
	scheme.Keywords = FlattenKeywords(theme.Keywords)

	return &scheme
}

func FlattenKeywords(keywords []styling.Keyword) []*core.Keyword {
	var flatKeywords []*core.Keyword

	for _, item := range keywords {
		for _, str := range item.Strings {

			var sAndR = core.Keyword{
				String: str,
				Fg:     item.Fg,
			}
			flatKeywords = append(flatKeywords, &sAndR)
		}
	}

	return flatKeywords
}
