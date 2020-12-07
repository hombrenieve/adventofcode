package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type node struct {
	color    string
	parents  []*node
	children []*childNode
	visited  bool
}

type childNode struct {
	number int
	child  *node
}

var nodes map[string]*node

func findNode(name string) *node {
	if found, ok := nodes[name]; ok {
		return found
	}
	newNode := &node{color: name}
	nodes[name] = newNode
	return newNode
}

func addChild(parent *node, number int, child *node) {
	newChild := &childNode{number, child}
	parent.children = append(parent.children, newChild)
}

func addParent(child *node, parent *node) {
	child.parents = append(child.parents, parent)
}

func countParents(current *node) int {
	fmt.Printf("Counting parents of %q\n", current.color)
	if current.visited {
		fmt.Println("Already visited")
		return 0
	}
	current.visited = true
	if len(current.parents) == 0 {
		fmt.Println("Returning one")
		return 1
	}
	var count int = 1
	for _, parent := range current.parents {
		count += countParents(parent)
	}
	fmt.Printf("Returning %d\n", count)
	return count
}

func main() {
	file, _ := os.Open("input")
	scanner := bufio.NewScanner(file)
	nodes = make(map[string]*node)

	for scanner.Scan() {
		buff := scanner.Text()
		nameAndRules := strings.Split(buff, " bags contain ")

		name := nameAndRules[0]

		rulesS := strings.Split(nameAndRules[1], ", ")
		if len(rulesS) == 1 {
			if rulesS[0] == "no other bags." {
				continue
			}
		}

		for _, rule := range rulesS {
			var adj, col string
			var number int
			fmt.Sscanf(rule, "%d %s %s", &number, &adj, &col)
			child := adj + " " + col

			parent := findNode(name)
			childNode := findNode(child)
			addChild(parent, number, childNode)
			addParent(childNode, parent)

			//fmt.Printf("Appending %d %q to %q\n", number, child, name)
		}

	}
	fmt.Printf("%d\n", countParents(findNode("shiny gold"))-1)
}
