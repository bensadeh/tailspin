package cmd

import (
	"github.com/spf13/cobra"
	"spin/app"
	"spin/file"
	"spin/settings"
)

var (
	debugMode bool
)

func Root() *cobra.Command {
	rootCmd := &cobra.Command{
		Use:     "spin {file}",
		Short:   "tailspin is a better way to tail your logfiles",
		Long:    "tailspin is a better way to tail your logfiles",
		Version: app.Version,
		Args:    cobra.MinimumNArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			_ = getConfig()

			file.Setup()
		},
	}

	rootCmd.CompletionOptions.DisableDefaultCmd = true

	rootCmd.AddCommand(versionCmd())
	rootCmd.AddCommand(debugCmd())

	configureFlags(rootCmd)

	return rootCmd
}

func configureFlags(rootCmd *cobra.Command) {
	rootCmd.PersistentFlags().BoolVarP(&debugMode, "debug-mode", "q", false,
		"debug tailspin by tailing a static log file")

	//rootCmd.PersistentFlags().BoolVarP(&debugMode, "debug-mode", "q", false,
	//	"enable debug mode (offline mode) by using mock data for the endpoints")
	//rootCmd.Flag("debug-mode").Hidden = true
}

func getConfig() *settings.Config {
	config := settings.New()

	config.DebugFile = debugFile

	return config
}
