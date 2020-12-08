package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type instruction struct {
	cmd      string
	arg      int
	executed bool
}

func executeUntilLoop(program []instruction, pi int, acc int) int {
	if program[pi].executed {
		return acc
	}
	program[pi].executed = true
	switch program[pi].cmd {
	case "nop":
		return executeUntilLoop(program, pi+1, acc)
	case "acc":
		return executeUntilLoop(program, pi+1, acc+program[pi].arg)
	case "jmp":
		return executeUntilLoop(program, pi+program[pi].arg, acc)
	}
	fmt.Println("PROGRAM ERROR")
	return -1
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	var program []instruction
	for scanner.Scan() {
		buff := scanner.Text()
		inst := strings.Split(buff, " ")
		cmd := inst[0]
		arg, _ := strconv.Atoi(inst[1])
		program = append(program, instruction{cmd, arg, false})
	}
	fmt.Printf("Program looped: acc: %v\n", executeUntilLoop(program, 0, 0))
}
