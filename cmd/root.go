package cmd

import (
	"github.com/spf13/cobra"
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
	rootCmd.PersistentFlags().BoolVarP(&follow, "follow", "f", false,
		"Scroll forward, and keep trying to read when the end of file is reached. "+
			"It is a way to monitor the tail of a file which is growing while it is being viewed. "+
			"(The behavior is similar to the \"tail -f\" command.)")
}

func getConfig() *settings.Config {
	config := settings.New()

	config.Follow = follow

	return config
}
