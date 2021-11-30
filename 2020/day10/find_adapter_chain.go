package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func getResult(input []int) int {
	sort.Ints(input)
	var differences [3]int
	previous := 0
	for _, current := range input {
		difference := current - previous
		differences[difference-1]++
		previous = current
	}
	fmt.Println(differences)
	return differences[0] * (differences[2] + 1)
}

func main() {
	var input []int
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		number, _ := strconv.Atoi(scanner.Text())
		input = append(input, number)
	}
	fmt.Println("Result:", getResult(input))
}
