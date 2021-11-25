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
}

func generatePoints(origin curr: inout Point, vector: Vector) -> Set<Point> {
    var points : Set<Point> = []
    var v = vector
    while v.amount > 0 {
        switch v.direction {
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
        v.amount-=1
        points.insert(curr)
    }
    return points
}

func loadLine(_ input: String) -> Set<Point> {
    var line : Set<Point> = []
    let steps = input.components(separatedBy: [","])
    var origin = Point(x: 0, y: 0)
    for step in steps {
        let v = Vector(direction: step.first!, amount: Int(step.substring(from: 1))!)
        let gp = generatePoints(origin: &origin, vector: v)
        line.formUnion(gp)
    }
    return line
}

let firstLine = loadLine(readLine()!)
let secondLine = loadLine(readLine()!)

let intersection = firstLine.intersection(secondLine)

print("Intersection: \(intersection))")

let minDistance = intersection.min { $0.manhattam() < $1.manhattam() }

print("Minimum: \(minDistance!.manhattam())")