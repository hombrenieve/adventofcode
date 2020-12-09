package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type preambleType struct {
	buffer   []int
	writePtr int
}

func newPreamble(size int) *preambleType {
	preamble := new(preambleType)
	preamble.buffer = make([]int, size)
	return preamble
}

func (p *preambleType) addElement(element int) {
	p.buffer[p.writePtr] = element
	p.writePtr = (p.writePtr + 1) % len(p.buffer)
}

func (p *preambleType) checkNumber(number int) bool {
	for i := 0; i < len(p.buffer); i++ {
		first := p.buffer[i]
		target := number - first
		for j := i + 1; j < len(p.buffer); j++ {
			if target == p.buffer[j] {
				return true
			}
		}
	}
	return false
}

func main() {
	file, _ := os.Open("input")
	preambleSize, err := strconv.Atoi(os.Args[1])
	if err != nil {
		fmt.Println("No preamble size provided considering 25")
		preambleSize = 25
	}
	scanner := bufio.NewScanner(file)
	var preamble = newPreamble(preambleSize)
	for i := 0; i < preambleSize; i++ {
		scanner.Scan()
		elem, _ := strconv.Atoi(scanner.Text())
		preamble.addElement(elem)
	}
	for scanner.Scan() {
		number, _ := strconv.Atoi(scanner.Text())
		if !preamble.checkNumber(number) {
			fmt.Printf("Number is invalid %d\n", number)
			break
		}
		preamble.addElement(number)
	}

}
