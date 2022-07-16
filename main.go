package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"os/exec"
	"path/filepath"
)

func main() {
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
