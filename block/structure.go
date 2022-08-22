package block

import "strings"

type Segment struct {
	Content   string
	Separator string
}

func ExtractSegments(text string) []*Segment {
	isOccurringEvenNumberOfTimes := strings.Count(text, `"`)%2 == 0
	if isOccurringEvenNumberOfTimes {
		return []*Segment{
			{
				Content:   text,
				Separator: "",
			},
		}
	}

	blocks := strings.Split(text, `"`)
	var segments []*Segment

	for i, block := range blocks {
		isInsideQuotesBlock := i%2 == 0

		segment := &Segment{
			Content:   block,
			Separator: getSeparatorType(isInsideQuotesBlock),
		}

		segments = append(segments, segment)
	}

	return segments
}

func getSeparatorType(isInsideQuotes bool) string {
	if isInsideQuotes {
		return `"`
	}

	return ""
}
