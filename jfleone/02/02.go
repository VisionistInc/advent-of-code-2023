package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

var redReg = regexp.MustCompile(`(\d+) red`)
var greenReg = regexp.MustCompile(`(\d+) green`)
var blueReg = regexp.MustCompile(`(\d+) blue`)

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

func getCounts(cubes string, reg *regexp.Regexp) int {

	var max int

	matches := reg.FindAllStringSubmatch(cubes, -1)

	for _, match := range matches {
		count, _ := strconv.Atoi(match[1])
		if count > max {
			max = count
		}
	}
	return max
}

func main() {
	lines := readLines("./input.txt")

    var sum int
	for i, line := range lines {

        r := getCounts(line, redReg)
        g := getCounts(line, greenReg)
        b := getCounts(line, blueReg)

        if (r <= 12 && g <= 13 && b <= 14) {
            sum += i + 1
        }
	}

    fmt.Println(sum)
}
