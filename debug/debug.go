package debug

import _ "embed"

//go:embed 1.log
var d1 string

//go:embed 2.log
var d2 string

//go:embed 3.log
var d3 string

func GetDebugFile(i int) string {
	switch i {
	case 1:
		return d1
	case 2:
		return d2
	case 3:
		return d3
	default:
		return ""
	}
}
