package settings

type Config struct {
	Follow    bool
	DebugMode bool
	DebugFile int
}

func New() *Config {
	return &Config{
		Follow:    false,
		DebugMode: false,
		DebugFile: 0,
	}
}
