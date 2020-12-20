package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type operationType struct {
	left      int
	right     int
	operation byte
}

func (op *operationType) solve() int {
	switch op.operation {
	case '+':
		return op.left + op.right
	case '*':
		return op.left * op.right
	default:
		return -536
	}
}

func calculate(operation string, pos int) (int, int) {
	var expr operationType

	for pos < len(operation) {
		switch byte(operation[pos]) {
		case ')':
			return expr.solve(), pos
		case '(':
			if expr.left == 0 {
				expr.left, pos = calculate(operation, pos+1)
			} else {
				expr.right, pos = calculate(operation, pos+1)
			}
		case '+':
			expr.operation = '+'
		case '*':
			expr.operation = '*'
		case ' ':
			if expr.left != 0 && expr.right != 0 {
				expr.left = expr.solve()
				expr.right = 0
			}
		default:
			if expr.left == 0 {
				expr.left, _ = strconv.Atoi(operation[pos : pos+1])
			} else if expr.right == 0 {
				expr.right, _ = strconv.Atoi(operation[pos : pos+1])
			}
		}
		pos++
	}
	return expr.solve(), pos
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	var accumulate int
	for scanner.Scan() {
		operations := scanner.Text()
		current, _ := calculate(operations, 0)
		accumulate += current
	}
	fmt.Println("Result:", accumulate)
}
