package cmd

import (
	"github.com/spf13/cobra"
	"os"
	"spin/app"
	"spin/file"
	"spin/settings"
)

var (
	follow bool
)

func Root() *cobra.Command {
	rootCmd := &cobra.Command{
		Use:     "spin {file}",
		Short:   "tailspin is a better way to browse your logfiles",
		Long:    "tailspin is a better way to browse your logfiles",
		Example: "spin system.log -f",
		Version: app.Version,
		Args:    cobra.MinimumNArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			config := getConfig()

			file.Setup(config, os.Args[1])
		},
	}

	rootCmd.CompletionOptions.DisableDefaultCmd = true

	rootCmd.AddCommand(versionCmd())
	rootCmd.AddCommand(debugCmd())

	configureFlags(rootCmd)

	return rootCmd
}

func configureFlags(rootCmd *cobra.Command) {
	rootCmd.PersistentFlags().BoolVarP(&follow, "follow", "f", false,
		"Scroll forward, and keep trying to read when the end of file is reached.\n"+
			"It is a way to monitor the tail of a file which is growing while it is\n"+
			"being viewed. (The behavior is similar to the \"tail -f\" command.)")

	// Flags and settings for debugging
	rootCmd.PersistentFlags().IntVar(&debugFile, "debug-file", 0,
		"select a specific log file for debugging")
	rootCmd.Flag("debug-file").Hidden = true

}

func getConfig() *settings.Config {
	config := settings.New()

	config.Follow = follow

	if debugFile != 0 {
		config.DebugMode = true
		config.DebugFile = debugFile
	}

	return config
}
