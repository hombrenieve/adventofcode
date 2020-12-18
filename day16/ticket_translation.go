package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type rangeRule struct {
	min int
	max int
}

type rule struct {
	lowPart  rangeRule
	highPart rangeRule
}

type rMap map[string]rule

func (rm *rMap) isValid(field int) bool {
	for _, r := range *rm {
		if !(field < r.lowPart.min ||
			field > r.highPart.max ||
			(field > r.lowPart.max && field < r.highPart.min)) {
			return true
		}
	}
	return false
}

func newRule(range1, range2 string) rule {
	stringToRange := func(rangeS string) rangeRule {
		parts := strings.Split(rangeS, "-")
		low, _ := strconv.Atoi(parts[0])
		high, _ := strconv.Atoi(parts[1])
		return rangeRule{low, high}
	}
	return rule{stringToRange(range1), stringToRange(range2)}
}

func createRuleMap(scanner *bufio.Scanner) rMap {
	ruleMap := make(map[string]rule)
	for scanner.Scan() {
		text := scanner.Text()
		if text == "" {
			return ruleMap
		}
		ruleText := strings.Split(text, ": ")
		name := ruleText[0]
		ruleParts := strings.Split(ruleText[1], " or ")
		ruleSt := newRule(ruleParts[0], ruleParts[1])
		ruleMap[name] = ruleSt
	}
	return ruleMap
}

func stringsToInt(stSlice []string) []int {
	var intSlice []int
	for _, v := range stSlice {
		intV, _ := strconv.Atoi(v)
		intSlice = append(intSlice, intV)
	}
	return intSlice
}

func inValidFields(ticket []int, ruleMap rMap) []int {
	var invalids []int
	for _, field := range ticket {
		if !ruleMap.isValid(field) {
			invalids = append(invalids, field)
		}
	}
	return invalids
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	ruleMap := createRuleMap(scanner)
	//Ignore until nearby tickets is shown
	for ; scanner.Text() != "nearby tickets:"; scanner.Scan() {
	}
	var errorRate int
	for scanner.Scan() {
		ticket := stringsToInt(strings.Split(scanner.Text(), ","))
		for _, v := range inValidFields(ticket, ruleMap) {
			errorRate += v
		}
	}
	fmt.Println("ErrorRate:", errorRate)
}
