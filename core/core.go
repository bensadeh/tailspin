package core

type Scheme struct {
	Keywords           []*Keyword
	RegularExpressions []*RegularExpression
	Date               []*Date
}

type Keyword struct {
	String string
	Fg     string
	Style  string
	Strict bool
}
type RegularExpression struct {
	RegExp string
	Fg     string
}
type Date struct {
	RegExp string
	Fg     string
}
