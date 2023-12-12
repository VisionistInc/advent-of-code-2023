package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strings"
)

const expansionFactor int64 = 1000000

type Galaxy struct {
	x, y int64
}

var galaxies []Galaxy

func readLines(path string) {
	file, _ := os.Open(path)
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	// The y is sorted naturally since we read line by line so expanding in place is easy
	var y int64 = 0

	for fileScanner.Scan() {
		line := fileScanner.Text()
		if !strings.ContainsRune(line, '#') {
			// no galaxies in row so expand
			y += expansionFactor
		} else {
			// contains galaxies
			for x, r := range line {
				if r == '#' {
					galaxies = append(galaxies, Galaxy{int64(x), int64(y)})
				}
			}
			y++
		}
	}
	file.Close()
}

func expandX() {
	// sort the galaxies by x first for easy xpansion
	slices.SortFunc(galaxies, func(a, b Galaxy) int {
		return int(a.x - b.x)
	})

	// Expand working left to right
	var xpansion int64 = 0
	for i := 1; i < len(galaxies); i++ {
		// expand current before calculating delta
		galaxies[i].x += xpansion
		d := (galaxies[i].x - galaxies[i-1].x)
		if d > 1 {
			inc := (expansionFactor - 1) * (d - 1)
			xpansion += inc
			// we have already expanded, but we need to expand more since the gap has widened
			galaxies[i].x += inc
		}
	}
}

func main() {
	readLines("./input.txt")

	var sum int64 = 0
	expandX()

	for i := 0; i < len(galaxies)-1; i++ {
		for j := i + 1; j < len(galaxies); j++ {
			sum += galaxies[j].x - galaxies[i].x // always positive due to sort

			dy := galaxies[j].y - galaxies[i].y
			if dy < 0 {
				sum -= dy
			} else {
				sum += dy
			}
		}
	}
	fmt.Println(sum)
}
