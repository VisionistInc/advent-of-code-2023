package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
)

type Node struct {
	L    string
	R    string
	name string
}

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

var reg = regexp.MustCompile(`(\w+)`)

func parse(line string) Node {
	parsed := reg.FindAllString(line, -1)
	return Node{parsed[1], parsed[2], parsed[0]}
}

var m map[string]Node

func main() {
	lines := readLines("./input.txt")

	// build map
	m = make(map[string]Node)

	for _, line := range lines[2:] {
		newNode := parse(line)
		m[newNode.name] = newNode
	}
	cur := m["AAA"]

	// instuction
	var steps int
	for cur.name != "ZZZ" {
		for _, c := range lines[0] {
			steps++
			if c == 'L' {
				cur = m[cur.L]
			} else if c == 'R' {
				cur = m[cur.R]
			}
		}
	}
	fmt.Println(steps)
}
