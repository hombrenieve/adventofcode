package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type calculable interface {
	solve() int
}

type operationType struct {
	left      calculable
	right     calculable
	operation byte
}

func (op *operationType) solve() int {
	if op.right == nil {
		return op.left.solve()
	}
	switch op.operation {
	case '+':
		return op.left.solve() + op.right.solve()
	case '*':
		return op.left.solve() * op.right.solve()
	default:
		return -536
	}
}

type intType struct {
	leaf int
}

func (op *intType) solve() int {
	return op.leaf
}

func build(operation string, pos int) (calculable, int) {
	var expr operationType

	for pos < len(operation) {
		switch byte(operation[pos]) {
		case ')':
			return &intType{expr.solve()}, pos
		case '(':
			if expr.left == nil {
				expr.left, pos = build(operation, pos+1)
			} else {
				expr.right, pos = build(operation, pos+1)
				expr = operationType{&intType{expr.solve()}, nil, 0}
			}
		case '+':
			expr.operation = '+'
		case '*':
			expr.operation = '*'
		case ' ':

		default:
			leaf, _ := strconv.Atoi(operation[pos : pos+1])
			if expr.left == nil {
				expr.left = &intType{leaf}
			} else if expr.right == nil {
				if expr.operation == '+' {
					expr.right = &intType{leaf}
				} else {
					expr.right, pos = build(operation, pos)
					if pos < len(operation) && byte(operation[pos]) == ')' {
						pos--
					}
				}
				expr = operationType{&intType{expr.solve()}, nil, 0}
			}
		}
		pos++
	}
	return &intType{expr.solve()}, pos
}

func calculate(operation calculable) int {
	return operation.solve()
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	var accumulate int
	for scanner.Scan() {
		operations := scanner.Text()
		current, _ := build(operations, 0)
		tmp := calculate(current)
		fmt.Println(operations, "=", tmp)
		accumulate += tmp
	}
	fmt.Println("Result:", accumulate)
}
