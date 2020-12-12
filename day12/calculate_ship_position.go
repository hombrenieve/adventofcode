package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

type position struct {
	x, y int
}

type location struct {
	pos position
	dir int //0-90-180-270
}

func (l *location) moveForward(units int) {
	switch l.dir {
	case 0:
		l.calculateNext('E', units)
	case 90:
		l.calculateNext('N', units)
	case 180:
		l.calculateNext('W', units)
	case 270:
		l.calculateNext('S', units)
	default:
		fmt.Println("Error in direction:", l.dir)
	}
}

func (l *location) calculateNext(cmd byte, arg int) {
	switch cmd {
	case 'F':
		l.moveForward(arg)
	case 'N':
		l.pos.y += arg
	case 'S':
		l.pos.y -= arg
	case 'W':
		l.pos.x -= arg
	case 'E':
		l.pos.x += arg
	case 'L':
		l.dir = (l.dir + arg) % 360
	case 'R':
		l.dir = (l.dir + (360 - arg)) % 360

	}
}

func (l *location) manhattam() int {
	return abs(l.pos.x) + abs(l.pos.y)
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	var ship location
	for scanner.Scan() {
		buf := scanner.Text()
		cmd := []byte(buf)[0]
		arg, _ := strconv.Atoi(buf[1:])
		//fmt.Println("Ship location:", ship)
		//fmt.Println("Cmd:", buf)
		ship.calculateNext(cmd, arg)
	}
	fmt.Println("Final state:", ship, "Manhattam:", ship.manhattam())

}
