package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func canIRemove(input []int) bool {
	return input[2]-input[0] < 4
}

func countOptions(input []int) int {
	if len(input) == 2 {
		return 1
	}
	if canIRemove(input[0:3]) {
		newInput := append([]int{input[0]}, input[2:]...)
		return countOptions(input[1:]) + countOptions(newInput)
	}
	return countOptions(input[1:])
}

func getCombinations(input []int) int {
	sort.Ints(input)
	newInput := append([]int{0}, input...)
	newInput = append(newInput, input[len(input)-1]+3)
	return countOptions(newInput)
}

func main() {
	var input []int
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		number, _ := strconv.Atoi(scanner.Text())
		input = append(input, number)
	}
	fmt.Println("Result:", getCombinations(input))
}
