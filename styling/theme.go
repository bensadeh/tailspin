package styling

import (
	_ "embed"
	"gopkg.in/yaml.v2"
	"log"
)

//go:embed defaults/defaults.yaml
var defaultTheme []byte

type Theme struct {
	SearchAndReplace  []SearchAndReplace  `yaml:"search_and_replace"`
	RegularExpression []RegularExpression `yaml:"regular_expression"`
	Date              Date                `yaml:"date"`
}
type SearchAndReplace struct {
	Keywords []string `yaml:"keywords"`
	Fg       string   `yaml:"fg"`
}
type RegularExpression struct {
	Regexp []string `yaml:"regexp"`
	Fg     string   `yaml:"fg"`
}
type Date struct {
	Regexp []string `yaml:"regexp"`
	Fg     string   `yaml:"fg"`
}

func GetTheme() *Theme {
	theme := Theme{}

	err := yaml.Unmarshal(defaultTheme, &theme)
	if err != nil {
		log.Fatalf("error: %v", err)
	}

	return &theme
}
