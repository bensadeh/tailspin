package settings

type Config struct {
	DebugFile int
}

func New() *Config {
	return &Config{
		DebugFile: 0,
	}
}
