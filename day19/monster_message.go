package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type rule struct {
	parts [2][]int
}

func (r rule) expand() []string {
	var theExpanse []string
	for i := 0; i < 2; i++ {
		var currentExpanse []string
		for _, rl := range r.parts[i] {
			if rl == ruleMap.ruleA {
				currentExpanse = mergeStrings(currentExpanse, []string{"a"})
			} else if rl == ruleMap.ruleB {
				currentExpanse = mergeStrings(currentExpanse, []string{"b"})
			} else {
				currentExpanse = mergeStrings(currentExpanse, ruleMap.theMap[rl].expand())
			}
		}
		theExpanse = append(theExpanse, currentExpanse...)
	}
	return theExpanse
}

func mergeStrings(current []string, news []string) (merged []string) {
	if len(current) == 0 {
		return news
	}
	for _, c := range current {
		for _, n := range news {
			merged = append(merged, c+n)
		}
	}
	return merged
}

func createRule(parts string) rule {
	var theRule rule
	currentPart := 0
	for _, part := range strings.Split(parts, " ") {
		if part == "|" {
			currentPart = 1
		} else {
			ruleNumber, _ := strconv.Atoi(part)
			theRule.parts[currentPart] = append(theRule.parts[currentPart], ruleNumber)
		}
	}
	return theRule
}

var ruleMap struct {
	theMap       map[int]rule
	ruleA, ruleB int
}

var expandedRules map[string]bool

func check(message string) bool {
	_, ok := expandedRules[message]
	return ok
}

func main() {
	file, _ := os.Open("input")
	ruleMap.theMap = make(map[int]rule)
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			break
		}
		lineS := strings.Split(line, ":")
		ruleNr, _ := strconv.Atoi(lineS[0])
		rulePart := strings.Trim(lineS[1], " ")
		if rulePart == "\"a\"" {
			ruleMap.ruleA = ruleNr
			continue
		} else if rulePart == "\"b\"" {
			ruleMap.ruleB = ruleNr
			continue
		}
		newRule := createRule(rulePart)
		ruleMap.theMap[ruleNr] = newRule
	}
	fmt.Println("Rule for A:", ruleMap.ruleA, "Rule for B:", ruleMap.ruleB)
	fmt.Println(ruleMap)
	expandedRules = make(map[string]bool)
	expanse := ruleMap.theMap[0].expand()
	for _, k := range expanse {
		expandedRules[k] = true
	}
	fmt.Println("Expanded:", len(expandedRules))
	var matched int
	for scanner.Scan() {
		if check(scanner.Text()) {
			matched++
		}
	}
	fmt.Println("Matched:", matched)
}
