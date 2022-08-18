package debug

import _ "embed"

//go:embed 1.log
var d1 string

func GetDebugFile(i int) string {
	switch i {
	case i:
		return d1
	default:
		return ""
	}
}
