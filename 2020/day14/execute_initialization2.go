package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

const word = 36

func duplicate(values [][]byte) [][]byte {
	duplicated := make([][]byte, len(values))
	for i, p := range values {
		duplicated[i] = make([]byte, len(values[i]))
		copy(duplicated[i], p)
	}
	return duplicated
}

func appendFront(values [][]byte, char byte) {
	for i := range values {
		values[i] = append([]byte{char}, values[i]...)
	}
}

type cmd struct {
	pattern   []byte
	memValues [][]byte
	value     int64
}

func (c *cmd) applyMask(mask []byte) {
	for i := 0; i < len(c.pattern); i++ {
		bit := mask[i]
		if bit != '0' {
			c.pattern[i] = bit
		}
	}
}

func widen(value []byte) []byte {
	prefixSize := word - len(value)
	result := value
	for i := 0; i < prefixSize; i++ {
		result = append([]byte{'0'}, result...)
	}
	return result
}

func (c *cmd) calculateValues() {
	switch c.pattern[len(c.pattern)-1] {
	case 'X':
		c.memValues = append(c.memValues, []byte{'0'}, []byte{'1'})
	default:
		c.memValues = append(c.memValues, []byte{c.pattern[len(c.pattern)-1]})
	}

	for i := len(c.pattern) - 2; i >= 0; i-- {
		switch c.pattern[i] {
		case 'X':
			resultDup := duplicate(c.memValues)
			appendFront(c.memValues, '0')
			appendFront(resultDup, '1')
			c.memValues = append(c.memValues, resultDup...)
		default:
			appendFront(c.memValues, c.pattern[i])
		}
	}
}

func newCommand(command string, value string) cmd {
	address, _ := strconv.Atoi(strings.TrimRight(strings.TrimLeft(command, "mem["), "]"))
	addrPattern := widen([]byte(strconv.FormatInt(int64(address), 2)))
	valueInt, _ := strconv.Atoi(value)
	cmdObj := cmd{
		pattern: addrPattern,
		value:   int64(valueInt),
	}
	return cmdObj
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	var mask []byte
	var program map[int64]int64 = make(map[int64]int64)
	for scanner.Scan() {
		command := strings.Split(scanner.Text(), " = ")
		if command[0] == "mask" {
			mask = []byte(command[1])
		}
		if strings.HasPrefix(command[0], "mem") {
			nCommand := newCommand(command[0], command[1])
			nCommand.applyMask(mask)
			fmt.Println("Pattern:", string(nCommand.pattern))
			nCommand.calculateValues()
			for _, v := range nCommand.memValues {
				addr, _ := strconv.ParseInt(string(v), 2, 64)
				program[addr] = nCommand.value
			}
		}
	}
	var result int64
	for _, v := range program {
		result += v
	}
	fmt.Println(program)
	fmt.Println("Result:", result)
}
