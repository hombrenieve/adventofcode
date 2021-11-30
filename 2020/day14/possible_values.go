package main

import (
	"fmt"
	"os"
)

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

func calculateValues(pattern []byte) [][]byte {
	var result [][]byte

	switch pattern[len(pattern)-1] {
	case 'X':
		result = append(result, []byte{'0'}, []byte{'1'})
	default:
		result = append(result, []byte{pattern[len(pattern)-1]})
	}

	for i := len(pattern) - 2; i >= 0; i-- {
		switch pattern[i] {
		case 'X':
			resultDup := duplicate(result)
			appendFront(result, '0')
			appendFront(resultDup, '1')
			result = append(result, resultDup...)
		default:
			appendFront(result, pattern[i])
		}
	}
	return result
}

func main() {
	pattern := os.Args[1]
	fmt.Println(calculateValues([]byte(pattern)))
}
