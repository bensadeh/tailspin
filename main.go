package main

import (
	"bufio"
	"fmt"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/hpcloud/tail"
	"io/ioutil"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

type editorFinishedMsg struct{ err error }

func openEditor(m *model) tea.Cmd {
	content := getFileContent()

	///

	tp, err := ioutil.TempFile("", fmt.Sprintf("%s-", filepath.Base(os.Args[0])))
	if err != nil {
		log.Fatal("Could not create temporary file", err)
	}

	m.tmpFile = tp

	if _, err = m.tmpFile.WriteString(content); err != nil {
		log.Fatal("Unable to write to temporary file", err)
	}

	////////////////////////////////////////////////////////// Tail
	filePath := os.Args[1]

	file, tailErr := tail.TailFile(
		filePath, tail.Config{Follow: true, ReOpen: true})
	if tailErr != nil {
		panic(err)
	}

	m.tail = file

	go startTailing(m, m.tmpFile)
	////////////////////////////////////////////////////////// Tail

	c := WrapLess(m.tmpFile.Name()) //nolint:gosec

	return tea.ExecProcess(c, func(err error) tea.Msg {
		m.tail.Done()
		return editorFinishedMsg{err}
	})
}

func startTailing(m *model, file *os.File) {
	for line := range m.tail.Lines {
		_, _ = file.WriteString(line.Text + "\n")
	}
}

func getFileContent() string {
	filePath := os.Args[1]
	readFile, err := os.Open(filePath)

	if err != nil {
		panic(err)
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)
	var fileLines []string

	for fileScanner.Scan() {
		fileLines = append(fileLines, fileScanner.Text())
	}

	_ = readFile.Close()

	var b strings.Builder
	for _, line := range fileLines {
		b.WriteString(line + "\n")
	}

	return b.String()
}

func WrapLess(path string) *exec.Cmd {
	command := exec.Command("less",
		path,
		"--RAW-CONTROL-CHARS",
		"--ignore-case",
		"--tilde",
		"--use-color")

	command.Stdin = os.Stdin
	command.Stdout = os.Stdout

	return command
}

type model struct {
	hasStarted bool
	tail       *tail.Tail
	tmpFile    *os.File
}

func (m model) Init() tea.Cmd {
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg.(type) {
	case editorFinishedMsg:

		tpErr := m.tmpFile.Close()
		if tpErr != nil {
			panic(tpErr)
		}

		tailErr := m.tail.Stop()
		if tailErr != nil {
			panic(tailErr)
		}
		//m.tail.Done()
		//m.tail.Cleanup()

		return m, tea.Quit
	}

	if m.hasStarted {
		return m, nil
	}

	m.hasStarted = true
	return m, openEditor(&m)
}

func (m model) View() string {
	return ""
}

func main() {
	m := model{}
	if err := tea.NewProgram(m).Start(); err != nil {
		fmt.Println("Error running program:", err)
		os.Exit(1)
	}
}

func oldMain() {
	//filePath := os.Args[1]
	//readFile, err := os.Open(filePath)
	//
	//if err != nil {
	//	fmt.Println(err)
	//}
	//fileScanner := bufio.NewScanner(readFile)
	//fileScanner.Split(bufio.ScanLines)
	//var fileLines []string
	//
	//for fileScanner.Scan() {
	//	fileLines = append(fileLines, fileScanner.Text())
	//}
	//
	//_ = readFile.Close()
	//
	//for _, line := range fileLines {
	//	fmt.Println(line)
	//}

	tmpFile, err := ioutil.TempFile("", fmt.Sprintf("%s-", filepath.Base(os.Args[0])))
	if err != nil {
		log.Fatal("Could not create temporary file", err)
	}

	defer func(tmpFile *os.File) {
		err := tmpFile.Close()
		if err != nil {
			panic(err)
		}
	}(tmpFile)

	fmt.Println("Created temp file: ", tmpFile.Name())

	fmt.Println("Writing some data to the temp file")
	if _, err = tmpFile.WriteString("test data"); err != nil {
		log.Fatal("Unable to write to temporary file", err)
	} else {
		fmt.Println("Data should have been written")
	}

	fmt.Println("Writing more data to the temp file")
	if _, err = tmpFile.WriteString("\nnew test data"); err != nil {
		log.Fatal("Unable to write to temporary file", err)
	} else {
		fmt.Println("Data should have been written")
	}

	fmt.Println("Trying to read the temp file now")

	less(tmpFile.Name())

	//if err = s.Err(); err != nil {
	//	log.Fatal("error reading temp file", err)
	//}

	//t, _ := tail.TailFile("/Users/E0O/.dotfiles/local.log", tail.Config{Follow: true})
	//for line := range t.Lines {
	//	fmt.Println(line.Text)
	//}
}

func less(path string) {
	command := exec.Command("less",
		path,
		"--RAW-CONTROL-CHARS",
		"--ignore-case",
		"--tilde")

	command.Stdin = os.Stdin
	command.Stdout = os.Stdout
	command.Stderr = os.Stderr

	if err := command.Run(); err != nil {
		panic(err)
	}
}
