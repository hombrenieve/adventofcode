import Foundation

struct Point: Hashable {
    let x : Int
    let y : Int
    func hash(into hasher: inout Hasher) {
        hasher.combine(x)
        hasher.combine(y)
    }
}
struct Line {
    let start: Point
    let end: Point
    var isHorizontal: Bool {
        if start.x == end.x {
            return true
        }
        return false
    }
    var isVertical: Bool {
        if start.y == end.y {
            return true
        }
        return false
    }
    var points: Set<Point> {
        var myPoints: Set<Point> = []
        if isHorizontal {
            var sty = 0
            var endy = 0
            if start.y < end.y {
                sty = start.y
                endy = end.y
            }else {
                sty = end.y
                endy = start.y
            }
            for y in sty...endy {
                myPoints.insert(Point(x: start.x, y: y))
            }
        } else if isVertical {
            var stx = 0
            var endx = 0
            if start.x < end.x {
                stx = start.x
                endx = end.x
            }else {
                stx = end.x
                endx = start.x
            }
            for x in stx...endx {
                myPoints.insert(Point(x: x, y: start.y))
            }
        }
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

let intrs = calculateIntersections(input: input.filter({$0.start.x == $0.end.x || $0.start.y == $0.end.y}))


print("Points: \(intrs)")
print("Calculated: \(intrs.count)")
