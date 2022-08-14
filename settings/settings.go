package settings

type Config struct {
	DebugFile int
	Follow    bool
}

func New() *Config {
	return &Config{
		DebugFile: 0,
		Follow:    false,
	}
}
