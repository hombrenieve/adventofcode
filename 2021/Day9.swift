import Foundation

typealias LavaMap = [[Int]]

func quicksort<T: Comparable>(_ a: [T]) -> [T] {
    guard a.count > 1 else { return a }

    let pivot = a[a.count/2]
    let less = a.filter { $0 < pivot }
    let equal = a.filter { $0 == pivot }
    let greater = a.filter { $0 > pivot }

    return quicksort(less) + equal + quicksort(greater)
}

struct Point: Hashable, Equatable {
    let x : Int
    let y : Int
    func hash(into hasher: inout Hasher) {
        hasher.combine(x)
        hasher.combine(y)
    }
    static func +(lhs: Point, rhs: Point) -> Point {
        return Point(
            x: lhs.x + rhs.x,
            y: lhs.y + rhs.y
        )
    }
    static func ==(lhs: Point, rhs: Point) -> Bool {
        return lhs.x == rhs.x && lhs.y == rhs.y
    }
}

extension LavaMap {

    func isLow(_ p: Point) -> Bool {
        let (x,y) = (p.x, p.y)
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
        return 1+self[p.y][p.x]
    }

    func findBasinPoints(from: Point, checked: Set<Point>) -> Set<Point> {
        if checked.contains(from) {
            return []
        }
        let (x, y) = (from.x, from.y)
        guard x >= 0, 
            x < self[0].count, 
            y >= 0, 
            y < self.count,
            self[y][x] < 9
        else {
            return []
        }
        var newChecked = checked.union([from])
        newChecked.formUnion(findBasinPoints(from: Point(x: x-1, y: y), checked: newChecked))
        newChecked.formUnion(findBasinPoints(from: Point(x: x+1, y: y), checked: newChecked))
        newChecked.formUnion(findBasinPoints(from: Point(x: x, y: y-1), checked: newChecked))
        newChecked.formUnion(findBasinPoints(from: Point(x: x, y: y+1), checked: newChecked))
        return newChecked

    }

    var lowPoints: [Point] {
        var points: [Point] = []
        for y in 0...self.count-1 {
            for x in 0...self[y].count-1 {
                if isLow(Point(x: x, y: y)) {
                    points.append(Point(x: x, y: y))
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
print("Result lows: \(result)")

let basinSizes = quicksort(lp.map({inMap.findBasinPoints(from: $0, checked: [])}).map({$0.count}))
print("Basin sizes: \(basinSizes)")
let basinResults = basinSizes[basinSizes.count-1]*basinSizes[basinSizes.count-2]*basinSizes[basinSizes.count-3]
print("Basin results: \(basinResults)")
