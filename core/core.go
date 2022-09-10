package core

type Scheme struct {
	SearchAndReplace  []SearchAndReplace
	RegularExpression []RegularExpression
	Date              []Date
}

type SearchAndReplace struct {
	Keyword string
	Fg      string
}
type RegularExpression struct {
	Regexp string
	Fg     string
}
type Date struct {
	Regexp string
	Fg     string
}
