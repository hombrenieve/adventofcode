import Foundation

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

struct Line {
    let start: Point
    let end: Point
    var direction: Point {
        var xDir = 0
        var yDir = 0
        switch start.x {
        case let x where x < end.x:
            xDir = 1
        case let x where x > end.x:
            xDir = -1
        default:
            xDir = 0
        }
        switch start.y {
        case let y where y < end.y:
            yDir = 1
        case let y where y > end.y:
            yDir = -1
        default:
            yDir = 0
        }
        return Point(x: xDir, y: yDir)
    }
    var points: Set<Point> {
        var myPoints: Set<Point> = []
        var point = start
        repeat {
            myPoints.insert(point)
            point = point+direction
        } while point != end
        myPoints.insert(end)
        return myPoints
    }
    func intersection(other: Line) -> Set<Point> {
        return self.points.intersection(other.points)
    }
}

func intersections(of: Line, with: [Line]) -> Set<Point> {
    var inters: Set<Point> = []
    for l in with {
        inters.formUnion(of.intersection(other: l))
    }
    return inters
}

func calculateIntersections(input: [Line]) -> Set<Point> {
    var inters: Set<Point> = []
    for i in 0..<input.count-1 {
        print("Calculating intersection: \(i+1)/\(input.count-1)")
        let partial = intersections(of: input[i], with: Array(input[i+1...input.count-1]))
        inters.formUnion(partial)
    }
    return inters
}

var input: [Line] = []
while let line = readLine() {
    let points = line.components(separatedBy: " -> ")
    let point1 = points[0].components(separatedBy: ",")
    let point2 = points[1].components(separatedBy: ",")
    let currentLine = Line(start: Point(x: Int(point1[0])!, y: Int(point1[1])!),
                           end: Point(x: Int(point2[0])!, y: Int(point2[1])!))
    input.append(currentLine)
}

let intrs = calculateIntersections(input: input)


print("Points: \(intrs)")
print("Calculated: \(intrs.count)")
