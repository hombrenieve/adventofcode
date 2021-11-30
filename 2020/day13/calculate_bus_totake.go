package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func busesIdsConvert(busesS []string) []int {
	var buses []int
	for _, bus := range busesS {
		if bus != "x" {
			id, _ := strconv.Atoi(bus)
			buses = append(buses, id)
		}
	}
	return buses
}

func calculateBusToTake(arrival int, buses []int) (int, int) { //(id, time_to_wait)
	var id int
	timeToWait := arrival
	for _, nextBus := range buses {
		currentDelay := arrival % nextBus
		if currentDelay == 0 {
			return nextBus, 0
		}
		currentTimeToWait := nextBus - currentDelay
		if currentTimeToWait < timeToWait {
			timeToWait = currentTimeToWait
			id = nextBus
		}
	}
	return id, timeToWait
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	scanner.Scan()
	expectedArrival, _ := strconv.Atoi(scanner.Text())
	scanner.Scan()
	busesIds := busesIdsConvert(strings.Split(scanner.Text(), ","))
	fmt.Println("Expected:", expectedArrival, "Ids:", busesIds)
	id, ttw := calculateBusToTake(expectedArrival, busesIds)
	fmt.Println("Id:", id, "Time to wait:", ttw, "Result:", id*ttw)
}
