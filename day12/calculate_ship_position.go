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

func (p *position) relative(other position) position {
	return position{other.x - p.x, other.y - p.y}
}

func (p *position) add(other position) position {
	newPos := *p
	newPos.x += other.x
	newPos.y += other.y
	return newPos
}

func (p *position) trasp() {
	sw := p.x
	p.x = p.y
	p.y = sw
}

type location struct {
	pos      position
	waypoint position
}

func newLocation() *location {
	l := &location{
		waypoint: position{x: 10, y: 1},
	}
	return l
}

func (l *location) moveForward(units int) {
	relative := l.pos.relative(l.waypoint)
	l.pos.x += relative.x * units
	l.pos.y += relative.y * units
	l.waypoint.x += relative.x * units
	l.waypoint.y += relative.y * units
}

func (l *location) rotate(deg int) {
	relative := l.pos.relative(l.waypoint)
	switch deg {
	case 90:
		relative.trasp()
		relative.x = -relative.x
		l.waypoint = l.pos.add(relative)
	case 180:
		relative.x = -relative.x
		relative.y = -relative.y
		l.waypoint = l.pos.add(relative)
	case 270:
		relative.trasp()
		relative.y = -relative.y
		l.waypoint = l.pos.add(relative)
	default:
		fmt.Println("Error in direction:", deg)
	}
}

func (l *location) calculateNext(cmd byte, arg int) {
	switch cmd {
	case 'F':
		l.moveForward(arg)
	case 'N':
		l.waypoint.y += arg
	case 'S':
		l.waypoint.y -= arg
	case 'W':
		l.waypoint.x -= arg
	case 'E':
		l.waypoint.x += arg
	case 'L':
		l.rotate(arg)
	case 'R':
		l.rotate(360 - arg)
	}
}

func (l *location) manhattam() int {
	return abs(l.pos.x) + abs(l.pos.y)
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	ship := newLocation()
	for scanner.Scan() {
		buf := scanner.Text()
		cmd := []byte(buf)[0]
		arg, _ := strconv.Atoi(buf[1:])
		//fmt.Println("Ship location:", ship)
		//fmt.Println("Cmd:", buf)
		ship.calculateNext(cmd, arg)
	}
	fmt.Println("Final state:", *ship, "Manhattam:", ship.manhattam())

}
