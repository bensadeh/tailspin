package settings

type Config struct {
	DebugMode bool
}

func New() *Config {
	return &Config{
		DebugMode: false,
	}
}
