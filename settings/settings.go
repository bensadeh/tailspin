package settings

type Config struct {
	DebugMode bool
	DebugFile int
	Follow    bool
}

func New() *Config {
	return &Config{
		DebugMode: false,
		DebugFile: 0,
		Follow:    false,
	}
}
