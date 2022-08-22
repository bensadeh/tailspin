package debug

import _ "embed"

//go:embed 1.log
var d1 string

//go:embed 2.log
var d2 string

func GetDebugFile(i int) string {
	switch i {
	case 1:
		return d1
	case 2:
		return d2
	default:
		return ""
	}
}
