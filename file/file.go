package file

import (
	"fmt"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/hpcloud/tail"
	"io/ioutil"
	"log"
	"os"
	"path/filepath"
	"spin/handler"
	"spin/syntax"
)

func Setup() {
	m := new(handler.Model)

	tp, err := ioutil.TempFile("", fmt.Sprintf("%s-", filepath.Base(os.Args[0])))
	if err != nil {
		log.Fatal("Could not create temporary file", err)
	}

	m.TempFile = tp

	if _, err = m.TempFile.WriteString(""); err != nil {
		log.Fatal("Unable to write to temporary file", err)
	}

	////////////////////////////////////////////////////////// Tail
	filePath := os.Args[1]

	file, tailErr := tail.TailFile(
		filePath, tail.Config{Follow: true})
	if tailErr != nil {
		panic(err)
	}

	m.TailFile = file

	go func() {
		for line := range m.TailFile.Lines {
			syntaxHighlightedLine := syntax.Highlight(line.Text)
			_, _ = m.TempFile.WriteString(syntaxHighlightedLine + "\n")
		}
	}()
	////////////////////////////////////////////////////////// Tail

	if err := tea.NewProgram(m).Start(); err != nil {
		fmt.Println("Error running program:", err)
	}

	tpErr := m.TempFile.Close()
	if tpErr != nil {
		panic(tpErr)
	}

	tErr := m.TailFile.Stop()
	if tErr != nil {
		panic(tErr)
	}

}
