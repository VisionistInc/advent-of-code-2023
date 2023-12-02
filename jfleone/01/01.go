package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func readLines(path string) []string {

	file, _ := os.Open(path)

	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)
	var fileLines []string

	for fileScanner.Scan() {
		fileLines = append(fileLines, fileScanner.Text())
	}

	file.Close()

	return fileLines
}

func main() {
	lines := readLines("./input.txt")

	firstDigitReg := regexp.MustCompile(`^[a-xA-Z]*(\d)`)
	lastDigitReg := regexp.MustCompile(`(\d)[a-zA-Z]*$`)

	var sum int

	for _, line := range lines {
		firstMatch := firstDigitReg.FindStringSubmatch(line)[1]
		lastMatch := lastDigitReg.FindStringSubmatch(line)[1]

		firstDigit, _ := strconv.Atoi(firstMatch)
		lastDigit, _ := strconv.Atoi(lastMatch)

		sum += firstDigit * 10
		sum += lastDigit
	}
	fmt.Println(sum)
}
