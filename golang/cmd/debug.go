package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"

	"spin/debug"
	"spin/file"
	"spin/mapper"
	"spin/styling"
)

var debugFile int

func debugCmd() *cobra.Command {
	command := &cobra.Command{
		Use:     "debug [--file n]",
		Example: "spin debug --file=1",
		Short:   "debug tailspin",
		Hidden:  true,
		Run: func(cmd *cobra.Command, args []string) {
			config := getConfig()
			config.DebugMode = true
			config.DebugFile = debugFile

			if debugFile == 0 {
				println("Specify which debug file to use with '--file=n'")
				os.Exit(1)
			}

			tempDebugFile, _ := os.CreateTemp("", fmt.Sprintf("tailspin-debug-%d", config.DebugFile))
			content := debug.GetDebugFile(debugFile)
			_, _ = tempDebugFile.WriteString(content)

			defer tempDebugFile.Close()

			theme := styling.GetTheme()
			scheme := mapper.MapTheme(theme)

			file.Setup(config, tempDebugFile.Name(), scheme)
		},
	}

	command.PersistentFlags().IntVar(&debugFile, "file", 0,
		"select a specific log file for debugging")

	return command
}
