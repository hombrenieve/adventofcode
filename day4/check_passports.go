package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func newPassport(strpass string) map[string]string {
	p := make(map[string]string)
	fields := strings.Split(strpass, " ")
	for _, f := range fields {
		spField := strings.Split(f, ":")
		p[spField[0]] = spField[1]
	}
	return p
}

func verifyYears(min, max int, years string) bool {
	match, _ := regexp.MatchString("^([0-9]{4})$", years)
	year, _ := strconv.Atoi(years)
	return match && min <= year && year <= max
}

func verifyHeight(heights string) bool {
	suffix := heights[len(heights)-2:]
	height, _ := strconv.Atoi(heights[:len(heights)-2])
	switch suffix {
	case "in":
		return 59 <= height && height <= 76
	case "cm":
		return 150 <= height && height <= 193
	default:
		return false
	}
}

func verifyFields(p map[string]string) bool {
	if match, _ := regexp.MatchString("^([0-9]{9})$", p["pid"]); !match {
		return false
	}
	if !verifyYears(1920, 2002, p["byr"]) {
		return false
	}
	if !verifyYears(2010, 2020, p["iyr"]) {
		return false
	}
	if !verifyYears(2020, 2030, p["eyr"]) {
		return false
	}
	if !verifyHeight(p["hgt"]) {
		return false
	}
	if match, _ := regexp.MatchString("^#(([0-9]|[a-f]){6})$", p["hcl"]); !match {
		return false
	}
	if match, _ := regexp.MatchString("^(amb|blu|brn|gry|grn|hzl|oth)$", p["ecl"]); !match {
		return false
	}

	return true
}

func isValid(p map[string]string) bool {
	switch len(p) {
	case 8:
		return verifyFields(p)
	case 7:
		if _, ok := p["cid"]; !ok {
			return verifyFields(p)
		}
	default:
		return false
	}
	return false
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	var validPassports int
	var strpass string
	for scanner.Scan() {
		buff := scanner.Text()
		if len(buff) == 0 {
			p := newPassport(strings.TrimLeft(strpass, "\t \n"))
			if isValid(p) {
				validPassports++
			}
			strpass = ""
		} else {
			strpass = strpass + " " + buff
		}
	}
	fmt.Printf("Valids are %v\n", validPassports)
}
