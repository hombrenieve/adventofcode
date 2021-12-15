import Foundation

struct Point: Hashable, Equatable {
    let x : Int
    let y : Int
    init(_ x: Int, _ y: Int) {
        self.x = x
        self.y = y
    }
    func hash(into hasher: inout Hasher) {
        hasher.combine(x)
        hasher.combine(y)
    }
    static func +(lhs: Point, rhs: Point) -> Point {
        return Point(
            lhs.x + rhs.x,
            lhs.y + rhs.y
        )
    }
    static func ==(lhs: Point, rhs: Point) -> Bool {
        return lhs.x == rhs.x && lhs.y == rhs.y
    }
}


class Node {
    let weight: Int
    var visited = false
    var distance = Int.max

    init(_ weight: Int) {
        self.weight = weight
    }
}

struct CaveMap {
    let factor = 5
    let data: [[Int]]
    var discovered: Dictionary<Point, Node> = [:]

    init(_ data: [[Int]]) {
        self.data = data
    }

    private func createNode(_ pos: Point) -> Node {
        let truePoint = Point(pos.x%data[0].count, pos.y%data.count)
        let multiplier = pos.x/data[0].count + pos.y/data.count
        let weightAux = data[truePoint.y][truePoint.x]+multiplier
        let weight = weightAux > 9 ? (weightAux % 10)+1 : weightAux
        //print("Create node: (\(pos.x), \(pos.y)) mult: \(multiplier) orig: (\(truePoint.x), \(truePoint.y)) w: \(data[truePoint.y][truePoint.x]) -> \(weight)")
        return Node(weight)
    }

    mutating func node(_ pos: Point) -> Node? {
        let (x, y) = (pos.x, pos.y)
        guard x >= 0 && x < data[0].count*factor,
            y >= 0 && y < data.count*factor else {
                return nil
            }
        if let node = discovered[pos] {
            return node
        } else {
            let node = createNode(pos)
            discovered[pos] = node
            return node
        }
    }

    mutating func dijkstra() -> Int {
        let end = Point(data[0].count*factor-1, data.count*factor-1)
        var pos = Point(0,0)
        node(pos)!.visited = true
        node(pos)!.distance = 0

        var toVisit = [Point]()
        toVisit.append(pos)

        while ( !toVisit.isEmpty ) {
            
            toVisit = toVisit.filter{ $0 != pos }
            node(pos)!.visited = true
            for incs in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let newPos = Point(pos.x+incs.0, pos.y+incs.1)
                if let connected = node(newPos) {
                    let dist = node(pos)!.distance + connected.weight
                    if dist < connected.distance {
                        connected.distance = dist
                        toVisit.append(newPos)
                        if connected.visited == true {
                            connected.visited = false
                        }
                    }
                }
            }
            node(pos)!.visited = true
            if !toVisit.isEmpty {
                pos = toVisit.min(by: { node($0)!.distance < node($1)!.distance })!
            }
            if pos == end {
                return node(pos)!.distance
            }
        }
        return -1
    }
}


var inputData: [String] = []

while let input = readLine() {
    inputData.append(input)
}

var cave = CaveMap(inputData.map({$0.map({$0.wholeNumberValue!})}))

print("Found easiest: \(cave.dijkstra())")