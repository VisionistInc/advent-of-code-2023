package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
	"strings"
)

var reg = regexp.MustCompile(`(\d+)`)

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

// keep around for later
func mapAtoI(s []string) []int {
	mapped := make([]int, len(s))
	for i, v := range s {
		conv, err := strconv.Atoi(v)
		if err == nil {
			mapped[i] = conv
		}
	}
	return mapped
}

// Take the cards string and return the winning and card numbers
func getNumbers(card string) ([]int, []int) {

	singleSpaced := regexp.MustCompile(` +`).ReplaceAllLiteralString(card, " ")
	splitBar := strings.Split(singleSpaced, " | ")
	winningNumbers := strings.Split(splitBar[0], " ")[2:]
	cardNumbers := strings.Split(splitBar[1], " ")

	return mapAtoI(winningNumbers), mapAtoI(cardNumbers)
}

// Ripped from stackoverflow
func intersection(a []int, b []int) []int {
	m := make(map[int]uint8)
	for _, k := range a {
		m[k] |= (1 << 0)
	}
	for _, k := range b {
		m[k] |= (1 << 1)
	}

	var inAAndB, inAButNotB, inBButNotA []int
	for k, v := range m {
		a := v&(1<<0) != 0
		b := v&(1<<1) != 0
		switch {
		case a && b:
			inAAndB = append(inAAndB, k)
		case a && !b:
			inAButNotB = append(inAButNotB, k)
		case !a && b:
			inBButNotA = append(inBButNotA, k)
		}
	}
	// all i care about today
	return inAAndB
}

func main() {
	lines := readLines("./input.txt")
	var sum int

	for _, line := range lines {

		winningNumbers, cardNumbers := getNumbers(line)

		matchCount := len(intersection(winningNumbers, cardNumbers))

		if matchCount > 0 {
			score := int(math.Pow(2, float64(matchCount-1)))
			sum += score
		}
	}

	fmt.Println(sum)
}
