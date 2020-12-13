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
	return buses, sorted[len(sorted)-1]
}

func checkConditions(t int64, buses map[int]int) bool {
	for id, offset := range buses {
		if (t+int64(offset))%int64(id) != 0 {
			return false
		}
	}
	return true
}

func calculateBusTime(buses map[int]int, biggest int) int64 {
	t := int64(biggest - buses[biggest])
	for ; !checkConditions(t, buses); t += int64(biggest) {
		//fmt.Println("Checking:", t)
	}
	return t
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	scanner.Scan()
	scanner.Scan()
	busesIds, biggest := busesIdsConvert(strings.Split(scanner.Text(), ","))
	fmt.Println("Ids:", busesIds, "Biggest:", biggest)
	fmt.Println("Timestamp:", calculateBusTime(busesIds, biggest))
}
