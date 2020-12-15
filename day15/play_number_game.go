package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type gameStatus struct {
	lastTurn     int
	previousTurn int
}

func main() {
	gameMap := make(map[int]*gameStatus)
	gameTurn := 1
	lastNumber := 0
	for _, numbS := range strings.Split(os.Args[1], ",") {
		numb, _ := strconv.Atoi(numbS)
		gameMap[numb] = &gameStatus{gameTurn, -1}
		gameTurn++
		lastNumber = numb
	}
	targetTurn, _ := strconv.Atoi(os.Args[2])
	for ; gameTurn <= targetTurn; gameTurn++ {
		if status, ok := gameMap[lastNumber]; !ok {
			gameMap[lastNumber] = &gameStatus{gameTurn - 1, -1}
			lastNumber = 0
		} else {
			status.previousTurn = status.lastTurn
			status.lastTurn = gameTurn - 1
			lastNumber = status.lastTurn - status.previousTurn
		}
	}
	fmt.Println("Turn:", gameTurn-1, "Number:", lastNumber)
}
