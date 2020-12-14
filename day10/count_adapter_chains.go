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

func countOptions(input []int) int64 {
	if len(input) == 2 {
		return 1
	}
	if canIRemove(input[0:3]) {
		newInput := append([]int{input[0]}, input[2:]...)
		return countOptions(input[1:]) + countOptions(newInput)
	}
	return countOptions(input[1:])
}

func divideAndCalculate(input []int) int64 {
	next := 0
	var results []int64
	for i := 0; i < len(input); i++ {
		if !canIRemove(input[i : i+3]) {
			if next != i {
				results = append(results, countOptions(input[next:i+2]))
			}
			next = i + 1
		}
	}
	var result int64 = 1
	for _, v := range results {
		result *= int64(v)
	}
	return result
}

func getCombinations(input []int) int64 {
	sort.Ints(input)
	newInput := append([]int{0}, input...)
	newInput = append(newInput, input[len(input)-1]+3)
	return divideAndCalculate(newInput)
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
