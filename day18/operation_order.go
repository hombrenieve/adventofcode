package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func reduce(singleExpr string) int {
	splitted := strings.Split(singleExpr, " ")
	left, _ := strconv.Atoi(splitted[0])
	right, _ := strconv.Atoi(splitted[2])
	switch splitted[1] {
	case "+":
		return left + right
	case "*":
		return left * right
	}
	return -435
}

func getExprLimits(expr string, operInd int) (int, int) {
	inf := 0
	sup := len(expr)
	for i := operInd - 2; i > 0; i-- {
		if expr[i] == ' ' {
			inf = i + 1
			break
		}
	}
	for i := operInd + 2; i < len(expr); i++ {
		if expr[i] == ' ' {
			sup = i
			break
		}
	}
	return inf, sup
}

func evaluate(expr string) int {
	//fmt.Println("Evaluate:", expr)
	operators := []string{"+", "*"}
	for _, op := range operators {
		for {
			ind := strings.Index(expr, op)
			if ind == -1 {
				break
			}
			inf, sup := getExprLimits(expr, ind)
			res := reduce(expr[inf:sup])
			resS := fmt.Sprintf("%d", res)
			expr = strings.ReplaceAll(expr, expr[inf:sup], resS)
			//fmt.Println("Reduced to:", expr)
		}
	}
	result, _ := strconv.Atoi(expr)
	return result
}

func findPars(expr string) (int, int) {
	inf := 0
	sup := len(expr)
	for i, c := range expr {
		if c == '(' {
			inf = i
		}
		if c == ')' {
			sup = i + 1
			break
		}
	}
	return inf, sup
}

func reduceExpr(expr string) int {
	//fmt.Println("ReduceExpr:", expr)
	for {
		inf, sup := findPars(expr)
		if inf == 0 && sup == len(expr) {
			break
		}
		res := evaluate(expr[inf+1 : sup-1])
		resS := fmt.Sprintf("%d", res)
		expr = strings.ReplaceAll(expr, expr[inf:sup], resS)
		//fmt.Println("Reduced to:", expr)
	}
	return evaluate(expr)
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	var accumulate int
	for scanner.Scan() {
		operations := scanner.Text()
		tmp := reduceExpr(operations)
		fmt.Println(operations, "=", tmp)
		accumulate += tmp
	}
	fmt.Println("Result:", accumulate)
}
