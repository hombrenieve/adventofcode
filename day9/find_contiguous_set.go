package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func findContiguousSet(preamble []int, target int) []int {
	if len(preamble) == 0 {
		return nil
	}
	if preamble[0] == target {
		return []int{preamble[0]}
	}
	if preamble[0] > target {
		return nil
	}
	newTarget := target - preamble[0]
	subset := findContiguousSet(preamble[1:], newTarget)
	if subset != nil {
		return append(subset, preamble[0])
	}
	return nil
}

func findSet(preamble []int, target int) []int {
	for i := 0; i < len(preamble); i++ {
		set := findContiguousSet(preamble[i:], target)
		if set != nil {
			return set
		}
	}
	return nil
}

func findMinMaxSum(result []int, target int) int {
	var min int = target
	var max int
	for _, numb := range result {
		if min > numb {
			min = numb
		}
		if max < numb {
			max = numb
		}
	}
	return min + max
}

func main() {
	file, _ := os.Open("input")
	target, _ := strconv.Atoi(os.Args[1])
	scanner := bufio.NewScanner(file)
	var preamble []int
	for scanner.Scan() {
		number, _ := strconv.Atoi(scanner.Text())
		if number != target {
			preamble = append(preamble, number)
		} else {
			break
		}
	}
	result := findSet(preamble, target)
	fmt.Printf("Found: %v\n", result)
	fmt.Printf("Sum %d\n", findMinMaxSum(result, target))
}
