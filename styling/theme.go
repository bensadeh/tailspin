package styling

import (
	_ "embed"
	"gopkg.in/yaml.v2"
	"log"
)

//go:embed defaults/defaults.yaml
var defaultTheme []byte

type Theme struct {
	Keywords           []Keyword           `yaml:"keyword"`
	RegularExpressions []RegularExpression `yaml:"regular_expression"`
	Date               Date                `yaml:"date"`
}
type Keyword struct {
	Strings []string `yaml:"strings"`
	Fg      string   `yaml:"fg"`
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
