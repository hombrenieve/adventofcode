package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type rule struct {
	parts [2][]int
}

func (r rule) regex(rm *ruleMap) string {
	regex := "("
	for i := 0; i < 2; i++ {
		var currentRegex string
		for _, rl := range r.parts[i] {
			if rl == rm.ruleA {
				currentRegex += "a"
			} else if rl == rm.ruleB {
				currentRegex += "b"
			} else {
				if rl == 8 {
					currentRegex += rm.theMap[42].regex(rm) + "+"
				} else if rl == 11 {
					rule42 := rm.theMap[42].regex(rm)
					rule31 := rm.theMap[31].regex(rm)
					currentRegex += "("
					for j := 1; j < 5; j++ {
						currentRegex += fmt.Sprintf("%s{%d}%s{%d}", rule42, j, rule31, j)
						currentRegex += "|"
					}
					currentRegex = currentRegex[:len(currentRegex)-1] + ")"
				} else {
					currentRegex += rm.theMap[rl].regex(rm)
				}
			}
		}
		if i == 0 && len(r.parts[1]) > 0 {
			regex += currentRegex + "|"
		} else {
			regex += currentRegex
		}
	}
	regex += ")"
	return regex
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

type ruleMap struct {
	theMap       map[int]rule
	ruleA, ruleB int
	compiled     *regexp.Regexp
}

func createRuleMap() *ruleMap {
	var rm ruleMap
	rm.theMap = make(map[int]rule)
	return &rm
}

func (rm *ruleMap) buildRegexp() {
	regex := "^" + rm.theMap[0].regex(rm) + "$"
	fmt.Println("Regex:", regex)
	rm.compiled = regexp.MustCompile(regex)

}

func (rm *ruleMap) check(message string) bool {
	return rm.compiled.MatchString(message)
}

func main() {
	file, _ := os.Open("input")
	rm := createRuleMap()
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
			rm.ruleA = ruleNr
			continue
		} else if rulePart == "\"b\"" {
			rm.ruleB = ruleNr
			continue
		}
		newRule := createRule(rulePart)
		rm.theMap[ruleNr] = newRule
	}
	fmt.Println("Rule for A:", rm.ruleA, "Rule for B:", rm.ruleB)
	rm.buildRegexp()
	var matched int
	for scanner.Scan() {
		if rm.check(scanner.Text()) {
			matched++
		}
	}
	fmt.Println("Matched:", matched)
}
