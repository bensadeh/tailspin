package file

import (
	"bufio"
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
		var wg sync.WaitGroup
		reader, _ := os.Open(pathToFileToBeTailed)
		numberOfLines, _ := lineCounter(reader)
		wg.Add(numberOfLines)

		////////////////////////////////////////////////////////// Tail
		file, tailErr := tail.TailFile(pathToFileToBeTailed, tail.Config{Follow: true})
		if tailErr != nil {
			panic(err)
		}

		m.TailFile = file

		go func() {
			currentLine := 0
			for line := range m.TailFile.Lines {
				syntaxHighlightedLine := syntax.Highlight(line.Text, scheme)
				_, _ = m.TempFile.WriteString(syntaxHighlightedLine + "\n")
				if currentLine < numberOfLines {
					wg.Done()
				}

				currentLine++
			}
		}()
		////////////////////////////////////////////////////////// Tail

		wg.Wait()
	}

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

//var wg sync.WaitGroup
//reader, _ := os.Open(pathToFileToBeTailed)
//numberOfLines, _ := lineCounter(reader)
//wg.Add(numberOfLines)
//
//////////////////////////////////////////////////////////// Tail
//file, tailErr := tail.TailFile(pathToFileToBeTailed, tail.Config{Follow: true})
//if tailErr != nil {
//panic(err)
//}
//
//m.TailFile = file
//
//go func() {
//	currentLine := 0
//	for line := range m.TailFile.Lines {
//		syntaxHighlightedLine := syntax.Highlight(line.Text, scheme)
//		_, _ = m.TempFile.WriteString(syntaxHighlightedLine + "\n")
//		if currentLine < numberOfLines {
//			wg.Done()
//		}
//
//		currentLine++
//	}
//}()
//////////////////////////////////////////////////////////// Tail
//
//wg.Wait()

func lineCounter(r io.Reader) (int, error) {

	var count int
	const lineBreak = '\n'

	buf := make([]byte, bufio.MaxScanTokenSize)

	for {
		bufferSize, err := r.Read(buf)
		if err != nil && err != io.EOF {
			return 0, err
		}

		var buffPosition int
		for {
			i := bytes.IndexByte(buf[buffPosition:], lineBreak)
			if i == -1 || bufferSize == buffPosition {
				break
			}
			buffPosition += i + 1
			count++
		}
		if err == io.EOF {
			break
		}
	}

	return count, nil
}
