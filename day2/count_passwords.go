package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func checkPassword(min, max int, letter, pass string) bool {
	counted := strings.Count(pass, letter)
	return counted >= min && counted <= max
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
		if checkPassword(min, max, letter, pass) {
			correct++
		}
	}
	fmt.Printf("Correct: %d\n", correct)
	file.Close()
}
