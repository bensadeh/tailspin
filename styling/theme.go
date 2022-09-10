package styling

import (
	_ "embed"
	"gopkg.in/yaml.v2"
	"log"
	"spin/core"
)

//go:embed defaults/defaults.yaml
var defaultTheme []byte

func GetTheme() *core.Theme {
	theme := core.Theme{}

	err := yaml.Unmarshal(defaultTheme, &theme)
	if err != nil {
		log.Fatalf("error: %v", err)
	}

	return &theme
}
