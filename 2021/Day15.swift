import Foundation

typealias Position = (Int, Int)


class Node {
    let weight: Int
    var visited = false
    var distance = Int.max

    init(_ weight: Int) {
        self.weight = weight
    }
}

struct CaveMap {
    let data: [[Node]]

    init(_ data: [[Int]]) {
        self.data = data.map({$0.map({Node($0)})})
    }

    func node(_ pos: Position) -> Node? {
        let (x, y) = pos
        guard x >= 0 && x < data[0].count,
            y >= 0 && y < data.count else {
                return nil
            }
        return data[y][x]
    }

    func dijkstra() -> Int {
        let end = (data[0].count-1, data.count-1)
        var pos = (0,0)
        node(pos)!.visited = true
        node(pos)!.distance = 0

        var toVisit = [Position]()
        toVisit.append(pos)

        while ( !toVisit.isEmpty ) {
            
            toVisit = toVisit.filter{ $0 != pos }
            node(pos)!.visited = true
            for incs in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let newPos = (pos.0+incs.0, pos.1+incs.1)
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

let cave = CaveMap(inputData.map({$0.map({$0.wholeNumberValue!})}))

print("Found easiest: \(cave.dijkstra())")