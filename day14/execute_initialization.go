package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

const word = 36

type cmd struct {
	address int
	value   []byte
}

func (c *cmd) applyMask(mask []byte) {
	for i := 0; i < len(c.value); i++ {
		bit := mask[len(mask)-1-i]
		if bit != 'X' {
			c.value[len(c.value)-1-i] = bit
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

func newCommand(command string, value string) cmd {
	address, _ := strconv.Atoi(strings.TrimRight(strings.TrimLeft(command, "mem["), "]"))
	valueInt, _ := strconv.Atoi(value)
	cmdObj := cmd{
		address,
		widen([]byte(strconv.FormatInt(int64(valueInt), 2))),
	}
	return cmdObj
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	var mask []byte
	var program map[int]cmd = make(map[int]cmd)
	for scanner.Scan() {
		command := strings.Split(scanner.Text(), " = ")
		if command[0] == "mask" {
			mask = []byte(command[1])
		}
		if strings.HasPrefix(command[0], "mem") {
			nCommand := newCommand(command[0], command[1])
			nCommand.applyMask(mask)
			program[nCommand.address] = nCommand
		}
	}
	var result int64
	for _, v := range program {
		value, _ := strconv.ParseInt(string(v.value), 2, 64)
		result += value
		fmt.Println("Mem:", v.address, "Value:", value)
	}
	fmt.Println("Result:", result)
}
