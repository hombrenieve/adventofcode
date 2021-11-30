package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type position struct {
	x int
	y int
}

func checkTree(ch byte) bool {
	return ch == '#'
}

func countTrees(slopex, slopey int, pattern [][]byte) int {
	var trees int
	pos := position{
		x: slopex,
		y: slopey,
	}
	for pos.y < len(pattern) {
		if checkTree(pattern[pos.y][pos.x]) {
			trees++
		}
		pos.x = (pos.x + slopex) % len(pattern[0])
		pos.y += slopey
	}
	return trees
}

func getNum(param string) int {
	num, _ := strconv.Atoi(param)
	return num
}

func main() {
	var pattern [][]byte
	args := os.Args[1:]
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		buff := scanner.Bytes()
		copied := make([]byte, len(buff))
		copy(copied, buff)

		pattern = append(pattern, copied)

	}

	fmt.Printf("Trees %v\n", countTrees(getNum(args[0]), getNum(args[1]), pattern))
}
