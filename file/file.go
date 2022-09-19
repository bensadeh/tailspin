package file

import (
	"fmt"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/nxadm/tail"
	"log"
	"os"
	"path/filepath"
	"spin/conf"
	"spin/core"
	"spin/handler"
	"spin/syntax"
	"strings"
)

func Setup(config *conf.Config, pathToFileToBeTailed string, scheme *core.Scheme) {
	m := new(handler.Model)

	temporaryFile, err := os.CreateTemp("", fmt.Sprintf("%s-", filepath.Base(os.Args[0])))
	if err != nil {
		log.Fatal("Could not create temporary file", err)
	}

	m.TempFile = temporaryFile
	m.Config = config

	if _, err = m.TempFile.WriteString(""); err != nil {
		log.Fatal("Unable to write to temporary file", err)
	}

	if config.Follow {
		////////////////////////////////////////////////////////// Tail
		file, tailErr := tail.TailFile(pathToFileToBeTailed, tail.Config{Follow: true})
		if tailErr != nil {
			panic(err)
		}

		m.TailFile = file

		go func() {
			for line := range m.TailFile.Lines {
				syntaxHighlightedLine := syntax.Highlight(line.Text, scheme)
				_, _ = m.TempFile.WriteString(syntaxHighlightedLine + "\n")
			}
		}()
		////////////////////////////////////////////////////////// Tail
	} else {
		b, err := os.ReadFile(pathToFileToBeTailed)
		if err != nil {
			fmt.Print(err)
		}
		str := string(b) // convert content to a 'string'

		output := ""
		for _, line := range strings.Split(str, "\n") {
			syntaxHighlightedLine := syntax.Highlight(line, scheme)
			output += syntaxHighlightedLine + "\n"
		}

		_, _ = m.TempFile.WriteString(output)
	}

	if err := tea.NewProgram(m).Start(); err != nil {
		fmt.Println("Error running program:", err)
	}

	tpErr := m.TempFile.Close()
	if tpErr != nil {
		panic(tpErr)
	}

	if config.Follow {
		tErr := m.TailFile.Stop()
		if tErr != nil {
			panic(tErr)
		}
	}

}
