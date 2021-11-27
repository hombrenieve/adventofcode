import Foundation


struct Point : Hashable {
    var x: Int
    var y: Int

    static func ==(lhs: Point, rhs: Point) -> Bool {
        return lhs.x == rhs.x && lhs.y == rhs.y
    }

    func hash(into hasher: inout Hasher) {
        hasher.combine(x)
        hasher.combine(y)
    }

    func manhattam() -> Int {
        return abs(x)+abs(y)
    }
}

struct Vector {
    var direction: Character
    var amount: Int

    private func changePoint(_ curr: inout Point) {
        switch direction {
            case "U":
                curr.y+=1
            case "D":
                curr.y-=1
            case "R":
                curr.x+=1
            case "L":
                curr.x-=1
            default:
                print("KK")
        }
    }

    func generatePoints(origin curr: inout Point) -> [Point] {
        var points : [Point] = []
        for _ in 1...amount {
            changePoint(&curr)
            points.append(curr)
        }
        return points
    }
}

func loadLine(_ input: String) -> [Point] {
    let steps = input.components(separatedBy: [","])
    var origin = Point(x: 0, y: 0)
    return steps.flatMap({ Vector(direction: $0.first!, amount: Int($0.substring(from: 1))!).generatePoints(origin: &origin) })
}

func distanceTo(point: Point, in array: [Point], and secondArray: [Point]) -> Int {
    return array.firstIndex(of: point)!+secondArray.firstIndex(of: point)!+2
}

func findMinDistance(firstWire first: [Point], secondWire second: [Point], intersection: Set<Point>) -> Int {
    return intersection.map({distanceTo(point: $0, in: first, and: second)}).min()!
}

let firstLine = loadLine(readLine()!)
let firstLineSet = Set<Point>.init(firstLine)
let secondLine = loadLine(readLine()!)
let secondLineSet = Set<Point>.init(secondLine)

let intersection = firstLineSet.intersection(secondLineSet)

print("Intersection: \(intersection))")

let minDistance = intersection.min { $0.manhattam() < $1.manhattam() }

print("Minimum: \(minDistance!.manhattam())")

let bestPath = findMinDistance(firstWire: firstLine, secondWire: secondLine, intersection: intersection)

print("Distances: \(bestPath)")