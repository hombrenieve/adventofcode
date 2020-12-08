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

func executeUntilLoop(program []instruction, pi int, acc int, changed bool) (int, bool) {
	if pi == len(program) {
		fmt.Printf("Reach end of program at %d\n", pi)
		return acc, true
	}
	fmt.Printf("Executing %v with acc: %d\n", program[pi], acc)
	if program[pi].executed {
		fmt.Println("Loop detected")
		return -1, false
	}
	program[pi].executed = true
	var accRet int
	var status bool
	switch program[pi].cmd {
	case "nop":
		accRet, status = executeUntilLoop(program, pi+1, acc, changed)
		if !status && !changed {
			accRet, status = executeUntilLoop(program, pi+program[pi].arg, acc, true)
		}
	case "acc":
		accRet, status = executeUntilLoop(program, pi+1, acc+program[pi].arg, changed)
	case "jmp":
		accRet, status = executeUntilLoop(program, pi+program[pi].arg, acc, changed)
		if !status && !changed {
			accRet, status = executeUntilLoop(program, pi+1, acc, true)
		}
	}
	if !status {
		program[pi].executed = false
		fmt.Printf("Unvisiting %v\n", program[pi])
		return -1, false
	}
	return accRet, status
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
	acc, status := executeUntilLoop(program, 0, 0, false)
	fmt.Printf("Program status: %v acc: %v\n", status, acc)
}
