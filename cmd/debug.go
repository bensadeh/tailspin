package cmd

import (
	"fmt"
	"github.com/spf13/cobra"
	"spin/app"
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
			fmt.Println(app.Version)

			//config := getConfig()

			fmt.Println(debugFile)

			//_ = getConfig()
			//
			//file.Setup()
		},
	}

	command.PersistentFlags().IntVarP(&debugFile, "file", "e", 0,
		"debug tailspin by tailing a static log file")

	return command
}
