package file

import (
	"bytes"
	"fmt"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/nxadm/tail"
	"io"
	"log"
	"os"
	"path/filepath"
	"spin/conf"
	"spin/core"
	"spin/handler"
	"spin/syntax"
	"sync"
)

func Setup(config *conf.Config, pathToFileToBeTailed string, scheme *core.Scheme) {
	m := new(handler.Model)

	temporaryFile, createTempErr := os.CreateTemp("", fmt.Sprintf("%s-", filepath.Base(os.Args[0])))
	if createTempErr != nil {
		log.Fatal("Could not create temporary file", createTempErr)
	}

	m.TempFile = temporaryFile
	m.Config = config

	if _, writeErr := m.TempFile.WriteString(""); writeErr != nil {
		log.Fatal("Unable to write to temporary file", writeErr)
	}

	file, tailErr := tail.TailFile(pathToFileToBeTailed, tail.Config{Follow: true})
	if tailErr != nil {
		panic(tailErr)
	}

	m.TailFile = file

	beginTailingAndHighlighting(config.Follow, pathToFileToBeTailed, m, scheme)

	_, teaErr := tea.NewProgram(m).Run()
	if teaErr != nil {
		fmt.Println("Error running program:", teaErr)
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

func beginTailingAndHighlighting(follow bool, pathToFileToBeTailed string, m *handler.Model, scheme *core.Scheme) {
	reader, _ := os.Open(pathToFileToBeTailed)
	numberOfLines, _ := lineCounter(reader)
	var wg sync.WaitGroup
	wg.Add(numberOfLines)

	go func() {
		for line := range m.TailFile.Lines {
			syntaxHighlightedLine := syntax.Highlight(line.Text, scheme)
			_, _ = m.TempFile.WriteString(syntaxHighlightedLine + "\n")

			if line.Num <= numberOfLines {
				wg.Done()
			}
		}
	}()

	if !follow {
		wg.Wait()
	}
}

func lineCounter(r io.Reader) (int, error) {
	buf := make([]byte, 32*1024)
	count := 0
	lineSep := []byte{'\n'}

	for {
		c, err := r.Read(buf)
		count += bytes.Count(buf[:c], lineSep)

		switch {
		case err == io.EOF:
			return count, nil

		case err != nil:
			return count, err
		}
	}
}
