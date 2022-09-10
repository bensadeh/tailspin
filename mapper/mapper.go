package mapper

import (
	"spin/core"
	"spin/styling"
)

func MapTheme(theme *styling.Theme) *core.Scheme {
	scheme := core.Scheme{}
	scheme.SearchAndReplace = FlattenSearchAndReplace(theme.SearchAndReplace)

	return &scheme
}

func FlattenSearchAndReplace(searchAndReplace []styling.SearchAndReplace) []core.SearchAndReplace {
	var flatSearchAndReplace []core.SearchAndReplace

	for _, item := range searchAndReplace {
		for _, keyword := range item.Keywords {

			sAndR := core.SearchAndReplace{
				Keyword: keyword,
				Fg:      item.Fg,
			}

			flatSearchAndReplace = append(flatSearchAndReplace, sAndR)
		}
	}

	return flatSearchAndReplace
}
