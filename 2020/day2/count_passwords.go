package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func checkPassword(pos1, pos2 int, letter, pass string) bool {
	char1 := pass[pos1 : pos1+1]
	char2 := pass[pos2 : pos2+1]
	isChar1 := char1 == letter
	isChar2 := char2 == letter
	return isChar1 != isChar2
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	var correct int
	for scanner.Scan() {
		text := scanner.Text()
		var min, max int
		var letter, pass string
		text = strings.Replace(text, ":", "", 1)
		fmt.Sscanf(text, "%d-%d %s %s\n", &min, &max, &letter, &pass)
		if checkPassword(min-1, max-1, letter, pass) {
			correct++
		}
	}
	fmt.Printf("Correct: %d\n", correct)
	file.Close()
}
