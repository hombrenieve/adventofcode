package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func calculateSeatID(bp string) (int64, int64, int64) {
	rows, _ := strconv.ParseInt(strings.Replace(strings.Replace(bp[0:7], "F", "0", -1), "B", "1", -1), 2, 8)
	col, _ := strconv.ParseInt(strings.Replace(strings.Replace(bp[7:10], "L", "0", -1), "R", "1", -1), 2, 8)
	return rows, col, rows*8 + col
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	var max int64
	for scanner.Scan() {
		buff := scanner.Text()
		_, _, id := calculateSeatID(buff)
		if id > max {
			max = id
		}
	}
	fmt.Println(max)
}
