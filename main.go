package main

import (
	"fmt"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/hpcloud/tail"
	"io/ioutil"
	"log"
	"os"
	"os/exec"
	"path/filepath"
)

type editorFinishedMsg struct{ err error }

func openEditor(m *model) tea.Cmd {

	c := less(m.tempFile.Name())

	return tea.ExecProcess(c, func(err error) tea.Msg {
		return editorFinishedMsg{err}
	})
}

func less(path string) *exec.Cmd {
	command := exec.Command("less",
		"--RAW-CONTROL-CHARS",
		"--ignore-case",
		"+F", // similar to 'tail -f'
		path)

	command.Stdin = os.Stdin
	command.Stdout = os.Stdout

	return command
}

type model struct {
	hasStarted bool
	tailFile   *tail.Tail
	tempFile   *os.File
}

func (m model) Init() tea.Cmd {
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg.(type) {
	case editorFinishedMsg:
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
	m := new(model)

	tp, err := ioutil.TempFile("", fmt.Sprintf("%s-", filepath.Base(os.Args[0])))
	if err != nil {
		log.Fatal("Could not create temporary file", err)
	}

	m.tempFile = tp

	if _, err = m.tempFile.WriteString(""); err != nil {
		log.Fatal("Unable to write to temporary file", err)
	}

	////////////////////////////////////////////////////////// Tail
	filePath := os.Args[1]

	file, tailErr := tail.TailFile(
		filePath, tail.Config{Follow: true})
	if tailErr != nil {
		panic(err)
	}

	m.tailFile = file

	go func() {
		for line := range m.tailFile.Lines {
			_, _ = m.tempFile.WriteString(line.Text + "\n")
		}
	}()
	////////////////////////////////////////////////////////// Tail

	if err := tea.NewProgram(m).Start(); err != nil {
		fmt.Println("Error running program:", err)
	}

	fmt.Println("Finished running Bubble Tea")

	fmt.Println("Closing temp file stream...")
	tpErr := m.tempFile.Close()
	if tpErr != nil {
		panic(tpErr)
	}

	fmt.Println("Closing tail stream...")
	tErr := m.tailFile.Stop()
	if tErr != nil {
		panic(tErr)
	}
	//m.tailFile.Done()
	//m.tailFile.Cleanup()

}
