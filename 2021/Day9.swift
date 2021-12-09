import Foundation

typealias LavaMap = [[Int]]
typealias Point = (Int, Int)

extension LavaMap {

    func isLow(_ p: Point) -> Bool {
        let (x,y) = p
        if x > 0 && self[y][x-1] <= self[y][x] {
            return false
        }
        if y > 0 && self[y-1][x] <= self[y][x] {
            return false
        }
        if x < self[y].count-1 && self[y][x+1] <= self[y][x] {
            return false
        }
        if y < self.count-1 && self[y+1][x] <= self[y][x] {
            return false
        }
        return true
    }

    func riskLevel(_ p: Point) -> Int {
        return 1+self[p.1][p.0]
    }

    var lowPoints: [Point] {
        var points: [Point] = []
        for y in 0...self.count-1 {
            for x in 0...self[y].count-1 {
                if isLow((x,y)) {
                    points.append((x,y))
                }
            }
        }
        return points
    }
}


var inMap: LavaMap = []

while let input = readLine() {
    inMap.append(input.map({$0.wholeNumberValue!}))
}

let lp = inMap.lowPoints
print("Points: \(lp)")
let rl = lp.map({inMap.riskLevel($0)})
print("Risk levels: \(rl)")
let result = rl.reduce(0,+)
print("Result: \(result)")