package main

import (
	"bufio"
	"fmt"
	"os"
)

type coordinate struct {
	x, y, z, w int
}

func (c coordinate) activeNeighbours(s space) int {
	return 80 - len(c.inactiveNeighbours(s))
}

func (c coordinate) remainsActive(s space) bool {
	n := c.activeNeighbours(s)
	return n == 2 || n == 3
}

func (c coordinate) becomesActive(s space) bool {
	return c.activeNeighbours(s) == 3
}

func (c coordinate) inactiveNeighbours(s space) []coordinate {
	var n []coordinate
	for i := c.x - 1; i < c.x+2; i++ {
		for j := c.y - 1; j < c.y+2; j++ {
			for k := c.z - 1; k < c.z+2; k++ {
				for p := c.w - 1; p < c.w+2; p++ {
					if i == c.x && j == c.y && k == c.z && p == c.w {
						continue
					}
					newC := coordinate{i, j, k, p}
					if _, ok := s[newC]; !ok {
						n = append(n, newC)
					}
				}
			}
		}
	}
	return n
}

type space map[coordinate]bool

func newSpace() space {
	return make(map[coordinate]bool)
}

func nextSpace(s space) space {
	newS := newSpace()
	for cube := range s {
		if cube.remainsActive(s) {
			newS[cube] = true
		}
		for _, c := range cube.inactiveNeighbours(s) {
			if c.becomesActive(s) {
				newS[c] = true
			}
		}
	}
	return newS
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	mySpace := newSpace()
	var y int
	for scanner.Scan() {
		input := []byte(scanner.Text())
		for x := 0; x < len(input); x++ {
			if input[x] == '#' {
				mySpace[coordinate{x, y, 0, 0}] = true
			}
		}
		y++
	}
	for cycle := 1; cycle < 7; cycle++ {
		mySpace = nextSpace(mySpace)
	}
	fmt.Println("After boot:", len(mySpace))
}
