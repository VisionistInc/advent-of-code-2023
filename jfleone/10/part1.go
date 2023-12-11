package main

import (
	"bufio"
	"fmt"
	"os"
)

type Node struct {
	row  int
	col  int
	rank int
}

var lines []string
var visitedNodes []Node
var nodesToVisit []Node

func readLines(path string) {

	file, _ := os.Open(path)

	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		lines = append(lines, fileScanner.Text())
	}

	file.Close()
}

func findStart() Node {
	for r, row := range lines {
		for c, val := range row {
			if val == 'S' {
				return Node{r, c, 0}
			}
		}
	}
	return Node{0, 0, 0}
}

func hasBeenVisited(node Node) int {
	for _, n := range visitedNodes {
		if n.col == node.col && n.row == node.row {
			return n.rank
		}
	}
	return -1
}
func getNextValidNodes(current Node) ([]Node, int) {
	var nextValidNodes []Node

	var nextRank int
	if current.row > 0 {
		top := lines[current.row-1][current.col]
		prevRank := hasBeenVisited(Node{current.row - 1, current.col, 0})
		if top == '|' || top == '7' || top == 'F' || top == 'S' {
			if prevRank == -1 {
				nextValidNodes = append(nextValidNodes, Node{current.row - 1, current.col, 0})
			} else {
				nextRank = prevRank + 1
			}
		}
	}
	if current.row < len(lines)-1 {
		bot := lines[current.row+1][current.col]
		prevRank := hasBeenVisited(Node{current.row + 1, current.col, 0})
		if bot == '|' || bot == 'L' || bot == 'J' || bot == 'S' {
			if prevRank == -1 {
				nextValidNodes = append(nextValidNodes, Node{current.row + 1, current.col, 0})
			} else {
				nextRank = prevRank + 1
			}
		}
	}
	if current.col > 0 {
		left := lines[current.row][current.col-1]
		prevRank := hasBeenVisited(Node{current.row, current.col - 1, 0})
		if left == '-' || left == 'L' || left == 'F' || left == 'S' {
			if prevRank == -1 {
				nextValidNodes = append(nextValidNodes, Node{current.row, current.col - 1, 0})
			} else {
				nextRank = prevRank + 1
			}
		}
	}
	if current.col < len(lines[0])-1 {
		right := lines[current.row][current.col+1]
		prevRank := hasBeenVisited(Node{current.row, current.col + 1, 0})
		if right == '-' || right == '7' || right == 'J' || right == 'S' {
			if prevRank == -1 {
				nextValidNodes = append(nextValidNodes, Node{current.row, current.col + 1, 0})
			} else {
				nextRank = prevRank + 1
			}
		}
	}

	return nextValidNodes, nextRank
}

func visitNodes() {
	var nextToVisit []Node

	for _, toVisit := range nodesToVisit {
		nextValidNodes, rank := getNextValidNodes(toVisit)
		nextToVisit = append(nextToVisit, nextValidNodes...)

		isAlreadyVisited := false
		for i, n := range visitedNodes {
			if n.row == toVisit.row && n.col == toVisit.col {
				isAlreadyVisited = true
				if n.rank < rank {
					visitedNodes[i] = Node{toVisit.row, toVisit.col, rank}
				}
			}
		}
		if !isAlreadyVisited {
			visitedNodes = append(visitedNodes, Node{toVisit.row, toVisit.col, rank})
		}
	}

	if len(nextToVisit) == 0 {
		return
	}
	nodesToVisit = nextToVisit
	visitNodes()
}


func main() {
	readLines("./input.txt")
	start := findStart()
	visitedNodes = append(visitedNodes, start)
	firstNodesToVisit, _ := getNextValidNodes(start)
	nodesToVisit = firstNodesToVisit
	visitNodes()

	max := 0
	for _, n := range visitedNodes {
		if n.rank > max {
			max = n.rank
		}
	}
	fmt.Println(max)
}
