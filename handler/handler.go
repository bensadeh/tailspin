package handler

import (
	tea "github.com/charmbracelet/bubbletea"
	"github.com/hpcloud/tail"
	"os"
	"os/exec"
)

type editorFinishedMsg struct{ err error }

type Model struct {
	TailFile   *tail.Tail
	TempFile   *os.File
	hasStarted bool
}

func openEditor(m *Model) tea.Cmd {

	c := less(m.TempFile.Name())

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

func (m Model) Init() tea.Cmd {
	return nil
}

func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
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

func (m Model) View() string {
	return ""
}
