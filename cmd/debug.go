package cmd

import (
	"fmt"
	"github.com/spf13/cobra"
	"os"
	"spin/debug"
	"spin/file"
	"spin/mapper"
	"spin/styling"
)

var (
	debugFile int
)

func debugCmd() *cobra.Command {

	command := &cobra.Command{
		Use:    "debug [-file n]",
		Short:  "debug tailspin",
		Hidden: true,
		Run: func(cmd *cobra.Command, args []string) {
			config := getConfig()
			config.DebugMode = true
			config.DebugFile = debugFile

			tempDebugFile, _ := os.CreateTemp("", fmt.Sprintf("tailspin-debug-%d", config.DebugFile))
			content := debug.GetDebugFile(debugFile)
			_, _ = tempDebugFile.WriteString(content)

			theme := styling.GetTheme()
			scheme := mapper.MapTheme(theme)

			file.Setup(config, tempDebugFile.Name(), scheme)

			defer tempDebugFile.Close()
		},
	}

	command.PersistentFlags().IntVar(&debugFile, "file", 0,
		"select a specific log file for debugging")

	return command
}
