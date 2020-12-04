package main

import (
	"bufio"
	"fmt"
	"os"
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

func isValid(p map[string]string) bool {
	switch len(p) {
	case 8:
		return true
	case 7:
		if _, ok := p["cid"]; !ok {
			return true
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
