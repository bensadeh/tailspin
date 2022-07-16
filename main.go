package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	filePath := os.Args[1]
	readFile, err := os.Open(filePath)

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)
	var fileLines []string

	for fileScanner.Scan() {
		fileLines = append(fileLines, fileScanner.Text())
	}

	_ = readFile.Close()

	for _, line := range fileLines {
		fmt.Println(line)
	}

	fmt.Println(fileLines)

	//t, _ := tail.TailFile("/Users/E0O/.dotfiles/local.log", tail.Config{Follow: true})
	//for line := range t.Lines {
	//	fmt.Println(line.Text)
	//}
}
