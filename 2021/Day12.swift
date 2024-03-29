import Foundation

class Node {
    let name: String
    var paths: [Node] = []
    var isBig: Bool {
        for char in name {
            if char.isLowercase {
                return false
            }
        }
        return true
    }
    var visited: Bool = false
    static var visitedTwice: Node? = nil

    init(_ theName: String) {
        name = theName
    }

    func addPath(_ node: Node) {
        paths.append(node)
    }

    func canVisit() -> Bool {
        return self.isBig || self.visited == false || (self.visited == true && Node.visitedTwice == nil && name != "start")
    }

    func visit() {
        if(visited == true && self.isBig == false) {
            if Node.visitedTwice == nil {
                Node.visitedTwice = self
            }
        }
        visited = true
    }

    func unvisit() {        
        if Node.visitedTwice === self {
            Node.visitedTwice = nil
            return
        }
        visited = false
    }

    func findPaths() -> Int {
        if name == "end" {
            return 1
        }
        visit()
        var possibleWays = 0
        for next in paths {
            if next.canVisit() {
                possibleWays += next.findPaths()
            }
        }
        unvisit()
        return possibleWays
    }
}

struct Graph {

    var nodes : Dictionary<String, Node> = [:]

    init(_ input: [String]) {
        for line in input {
            let nodeNames = line.components(separatedBy: "-")
            let node1 = self.findOrInsert(nodeName: nodeNames[0])
            let node2 = self.findOrInsert(nodeName: nodeNames[1])
            node1.addPath(node2)
            node2.addPath(node1)
        }
    }

    mutating func findOrInsert(nodeName: String) -> Node {
        if let current = nodes[nodeName] {
            return current
        }
        let node = Node(nodeName)
        nodes[nodeName] = node
        return node
    }    

    func reset() {
        nodes.forEach({$1.visited = false})
    }

    func findPaths() -> Int {
        let start = nodes["start"]!
        return start.findPaths()
    }

    func printer() {
        for (_, el) in nodes {
            print("Node: \(el.name), isBig: \(el.isBig), Paths to: \(el.paths)")
        }
    }
}

var input: [String] = []

while let line = readLine() {
    input.append(line)
}

let graph = Graph(input)

print("Possible paths: \(graph.findPaths())")