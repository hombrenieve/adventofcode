package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
)

func checkEqual(step1, step2 [][]byte) bool {
	for i := 0; i < len(step1); i++ {
		if !bytes.Equal(step1[i], step2[i]) {
			return false
		}
	}
	return true
}

func occupiedInDirection(input [][]byte, x, y int, sumX, sumY int) int {
	i := x + sumX
	j := y + sumY
	for i >= 0 && i < len(input) && j >= 0 && j < len(input[i]) {
		if input[i][j] == '#' {
			return 1
		} else if input[i][j] == 'L' {
			return 0
		}
		i += sumX
		j += sumY
	}
	return 0
}

func occupiedAdjacent(input [][]byte, x, y int) int {
	busyPlaces := occupiedInDirection(input, x, y, -1, -1) +
		occupiedInDirection(input, x, y, -1, 0) +
		occupiedInDirection(input, x, y, -1, 1) +
		occupiedInDirection(input, x, y, 0, -1) +
		occupiedInDirection(input, x, y, 0, 1) +
		occupiedInDirection(input, x, y, 1, -1) +
		occupiedInDirection(input, x, y, 1, 0) +
		occupiedInDirection(input, x, y, 1, 1)
	return busyPlaces
}

func calculateNextState(input [][]byte) [][]byte {
	var current [][]byte
	for i := 0; i < len(input); i++ {
		row := make([]byte, len(input[i]), len(input[i]))
		for j := 0; j < len(input[i]); j++ {
			busy := occupiedAdjacent(input, i, j)
			switch {
			case busy == 0 && input[i][j] == 'L':
				row[j] = '#'
			case busy >= 5 && input[i][j] == '#':
				row[j] = 'L'
			default:
				row[j] = input[i][j]
			}
		}
		current = append(current, row)
	}
	return current
}

func print(board [][]byte) {
	for _, v := range board {
		fmt.Println(string(v))
	}
}

func countOccupied(input [][]byte) int {
	var occupied int
	for _, v := range input {
		occupied += bytes.Count(v, []byte{'#'})
	}
	return occupied
}

func main() {
	var input [][]byte
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		input = append(input, []byte(scanner.Text()))
	}
	var previous = input
	var rounds int
	for {
		current := calculateNextState(previous)
		if checkEqual(previous, current) {
			break
		}
		previous = current
		rounds++
	}
	fmt.Println("Rounds:", rounds)
	fmt.Println("Occupied:", countOccupied(previous))
}
