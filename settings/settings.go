package settings

type Config struct {
	Follow    bool
	DebugMode bool
	DebugType int
}

func New() *Config {
	return &Config{
		Follow:    false,
		DebugMode: false,
		DebugType: 0,
	}
}
