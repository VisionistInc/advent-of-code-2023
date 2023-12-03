package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

var reg = regexp.MustCompile(`(\d+)`)
var symbols = "!@#$%^&*_+-=/\\"

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

// Get the bounds for a "box" around the indices of the matched number string and check for symbols around it
func isPartNum(schematic []string, row int, match []int) bool {

	var start = match[0]
	var end = match[1]

	if start > 0 {
		start--
	}
	if end < len(schematic[0]) {
		end++
	}

	var startRow = row - 1
	if startRow < 0 {
		startRow = 0
	}

	var endRow = row + 2
	if endRow > len(schematic) {
		endRow = len(schematic)
	}

	for _, line := range schematic[startRow:endRow] {
		if strings.ContainsAny(line[start:end], symbols) {
			return true
		}
	}

	return false
}

func main() {
	lines := readLines("./input.txt")
	var sum int

	for row, line := range lines {
		for _, match := range reg.FindAllStringIndex(line, -1) {

			if isPartNum(lines, row, match) {
				num, _ := strconv.Atoi(line[match[0]:match[1]])
                sum += num
			}
		}
	}

	fmt.Println(sum)
}
