package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func busesIdsConvert(busesS []string) (map[int]int, int) {
	buses := make(map[int]int)
	var sorted []int
	for delay, bus := range busesS {
		if bus != "x" {
			id, _ := strconv.Atoi(bus)
			buses[id] = delay
			sorted = append(sorted, id)
		}
	}
	sort.Ints(sorted)
	return buses, sorted[0]
}

var agreed []int
var interval int64

func isContained(a int, b []int) bool {
	for _, v := range b {
		if a == v {
			return true
		}
	}
	return false
}

func contains(a, b []int) bool {
	for _, vb := range b {
		if !isContained(vb, a) {
			return false
		}
	}
	return true
}

func multAll(b []int) int64 {
	var m int64 = 1
	for _, v := range b {
		m *= int64(v)
	}
	return m
}

func checkConditions(t int64, buses map[int]int) bool {
	status := true
	var agreedNow []int
	for id, offset := range buses {
		if (t+int64(offset))%int64(id) != 0 {
			status = false
		} else {
			agreedNow = append(agreedNow, id)
		}
	}
	if len(agreedNow) > 1 {
		if len(agreedNow) > len(agreed) && contains(agreedNow, agreed) {
			interval = multAll(agreedNow)
			fmt.Println("New interval found at", t, "Numbers:", agreedNow, "Interval:", interval)
			agreed = agreedNow
		}
	}
	return status
}

func calculateBusTime(buses map[int]int, smallest int) int64 {
	interval = int64(smallest)
	t := int64(smallest)
	for ; !checkConditions(t, buses); t += interval {
		//fmt.Println("Checking:", t)
	}
	return t
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	scanner.Scan()
	scanner.Scan()
	busesIds, smallest := busesIdsConvert(strings.Split(scanner.Text(), ","))
	fmt.Println("Ids:", busesIds, "Biggest:", smallest)
	fmt.Println("Timestamp:", calculateBusTime(busesIds, smallest))
}
