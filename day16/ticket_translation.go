package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
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

func (r *rule) isValid(field int) bool {
	return !(field < r.lowPart.min ||
		field > r.highPart.max ||
		(field > r.lowPart.max && field < r.highPart.min))
}

type rMap map[string]*rule

func (rm *rMap) isValid(field int) bool {
	for _, r := range *rm {
		if r.isValid(field) {
			return true
		}
	}
	return false
}

func newRule(range1, range2 string) *rule {
	stringToRange := func(rangeS string) rangeRule {
		parts := strings.Split(rangeS, "-")
		low, _ := strconv.Atoi(parts[0])
		high, _ := strconv.Atoi(parts[1])
		return rangeRule{low, high}
	}
	return &rule{stringToRange(range1), stringToRange(range2)}
}

func createRuleMap(scanner *bufio.Scanner) rMap {
	ruleMap := make(map[string]*rule)
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

func isValid(ticket []int, ruleMap rMap) bool {
	for _, field := range ticket {
		if !ruleMap.isValid(field) {
			return false
		}
	}
	return true
}

func processTicket(ticket []int, r *rule, validFields []bool) {
	for i, field := range ticket {
		//Check field is correct
		if !r.isValid(field) {
			validFields[i] = false
		}
	}
}

type posRow []bool

func (pr *posRow) findNext(pos int) int {
	for i := pos; i < len(*pr); i++ {
		if (*pr)[i] {
			return i
		}
	}
	return -1
}

func (pr *posRow) numOfPositives() int {
	var n int
	for _, v := range *pr {
		if v {
			n++
		}
	}
	return n
}

type posHint struct {
	name          string
	possibilities posRow
}

type orderedNames []string

func (rm *orderedNames) contains(i int) bool {
	return (*rm)[i] != ""
}

func processRule(r *rule, tickets [][]int) posRow {
	isValidField := make([]bool, len(tickets[0]), len(tickets[0]))
	for i := range isValidField {
		isValidField[i] = true
	}
	for _, ticket := range tickets {
		processTicket(ticket, r, isValidField)
	}
	return isValidField
}

func findResult(possibilities []posHint, results orderedNames) bool {
	if len(possibilities) == 0 {
		return true
	}
	current := possibilities[0]
	for i := 0; i < len(current.possibilities); {
		nextPos := current.possibilities.findNext(i)
		if nextPos == -1 {
			break
		}
		if results.contains(nextPos) {
			i = nextPos + 1
			continue
		}
		results[nextPos] = current.name
		found := findResult(possibilities[1:], results)
		if !found {
			results[nextPos] = ""
			i = nextPos + 1
		} else {
			return true
		}
	}
	return false
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	ruleMap := createRuleMap(scanner)
	//Ignore until nearby tickets is shown
	scanner.Scan()
	scanner.Scan()
	yourTicket := stringsToInt(strings.Split(scanner.Text(), ","))
	for ; scanner.Text() != "nearby tickets:"; scanner.Scan() {
	}
	var tickets [][]int
	for scanner.Scan() {
		ticket := stringsToInt(strings.Split(scanner.Text(), ","))
		if isValid(ticket, ruleMap) {
			tickets = append(tickets, ticket)
		}

	}
	tickets = append(tickets, yourTicket)

	var possibilities []posHint
	for n, r := range ruleMap {
		possibilities = append(possibilities,
			posHint{
				n,
				processRule(r, tickets),
			})
	}
	sort.Slice(possibilities, func(i, j int) bool {
		return possibilities[i].possibilities.numOfPositives() < possibilities[j].possibilities.numOfPositives()
	})
	fmt.Println(possibilities)

	results := make([]string, len(ruleMap), len(ruleMap))
	found := findResult(possibilities, results)
	fmt.Println("Found:", found, "Results:", results)

	result := 1
	fmt.Println("-----------------------")
	for i, v := range yourTicket {
		fmt.Printf("%s: %d\n", results[i], v)
		if strings.HasPrefix(results[i], "departure") {
			result *= v
		}
	}
	fmt.Println("Result:", result)
	fmt.Println("-----------------------")
}
