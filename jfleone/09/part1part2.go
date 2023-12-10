package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func readLines(path string) [][]int {

	file, _ := os.Open(path)

	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)
	var fileLines [][]int

	for fileScanner.Scan() {
		line := strings.Split(fileScanner.Text(), " ")
		iarr := make([]int, len(line))
		for i := 0; i < len(line); i++ {
			toInt, _ := strconv.Atoi(line[i])
			iarr[i] = toInt
		}
		fileLines = append(fileLines, iarr)
	}

	file.Close()

	return fileLines
}

func recurse(in []int) [][]int {

	var differences []int
	allZeros := true
	for i := 1; i < len(in); i++ {
		diff := in[i] - in[i-1]
		differences = append(differences, diff)
		if diff != 0 {
			allZeros = false
		}
	}

	if allZeros {
		return [][]int{differences, in}
	}
	return append(recurse(differences), in)
}

// Part 1
func extrapolate(hist []int) int {

	pattern := recurse(hist)


	for i := 1; i < len(pattern); i++ {
		data := pattern[i]
		nextVal := data[len(data)-1] + pattern[i-1][len(pattern[i-1])-1]
		pattern[i] = append(pattern[i], nextVal)
	}
	return pattern[len(pattern)-1][len(hist)]
}

// Part 2
func extrapolateBackwards(hist []int) int {
	pattern := recurse(hist)

	for i := 1; i < len(pattern); i++ {
		data := pattern[i]
		nextVal := data[0] - pattern[i-1][0]
		pattern[i] = append([]int{nextVal}, pattern[i]...)
	}
	return pattern[len(pattern)-1][0]
}

func main() {
	lines := readLines("./input.txt")

	sum := 0
	for _, line := range lines {
		// sum += extrapolate(line)
		sum += extrapolateBackwards(line)
		fmt.Println(sum)
	}
}
