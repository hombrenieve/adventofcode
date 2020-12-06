package main

import (
	"bufio"
	"fmt"
	"os"
)

func calculateNumberOfValidQuestions(questionaire string) int {
	validQuestions := make(map[byte]bool)
	for _, q := range questionaire {
		validQuestions[byte(q)] = true
	}
	return len(validQuestions)
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	var groupQuestionaire string
	var validQuestions int
	for scanner.Scan() {
		buff := scanner.Text()
		if len(buff) != 0 {
			groupQuestionaire = groupQuestionaire + buff
		} else {
			validQuestions += calculateNumberOfValidQuestions(groupQuestionaire)
			groupQuestionaire = ""
		}
	}
	fmt.Println(validQuestions)
}
