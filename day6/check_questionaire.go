package main

import (
	"bufio"
	"fmt"
	"os"
)

func calculateNumberOfValidQuestions(questionaire string, members int) int {
	validQuestions := make(map[byte]int)
	var valids int
	for _, q := range questionaire {
		bq := byte(q)
		if _, ok := validQuestions[bq]; ok {
			validQuestions[bq]++
		} else {
			validQuestions[bq] = 1
		}
		if validQuestions[bq] == members {
			valids++
		}
	}
	return valids
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	var groupQuestionaire string
	var members int
	var validQuestions int
	for scanner.Scan() {
		buff := scanner.Text()
		if len(buff) != 0 {
			groupQuestionaire = groupQuestionaire + buff
			members++
		} else {
			validQuestions += calculateNumberOfValidQuestions(groupQuestionaire, members)
			groupQuestionaire = ""
			members = 0
		}
	}
	fmt.Println(validQuestions)
}
