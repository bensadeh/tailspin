package cmd

import (
	"fmt"
	"github.com/spf13/cobra"
	"spin/app"
)

func debugCmd() *cobra.Command {
	return &cobra.Command{
		Use:    "debug",
		Short:  "debug tailspin",
		Hidden: true,
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(app.Version)

			config := getConfig()

			fmt.Println(config.DebugMode)

			//_ = getConfig()
			//
			//file.Setup()
		},
	}
}
