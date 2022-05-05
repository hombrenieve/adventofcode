import Foundation

struct Point:  Hashable, Equatable {
    let x : Int
    let y : Int
    let z : Int
    init(_ x: Int, _ y: Int, _ z: Int) {
        self.x = x
        self.y = y
        self.z = z
    }

    func hash(into hasher: inout Hasher) {
        hasher.combine(abs(self.x))
        hasher.combine(abs(self.y))
        hasher.combine(abs(self.z))
    }

    static func -(lhs: Point, rhs: Point) -> Point {
        return Point(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z)
    }

    static func +(lhs: Point, rhs: Point) -> Point {
        return Point(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z)
    }

    func transformation(to: Point) -> Point {
        return Point(abs(to.x)-abs(x), abs(to.y)-abs(y), abs(to.z)-abs(z))
    }

    static func ==(lhs: Point, rhs: Point) -> Bool {
        return abs(lhs.x) == abs(rhs.x) && abs(lhs.y) == abs(rhs.y) && abs(lhs.z) == abs(rhs.z)
    }
}

struct ScannerMap {
    let id: Int
    var origin: Point
    var points: [Point]

    init(id: Int) {
        self.id = id
        self.origin = Point(0, 0, 0)
        self.points = []
        while let input = readLine() {
            if input == "" {
                break
            }
            let comps = input.components(separatedBy: ",")
            self.points.append(Point(Int(comps[0])!, Int(comps[1])!, Int(comps[2])!))
        }
    }

    mutating func moveOrigin(to: Point) {
        let transformation = to - self.origin
        self.origin = to
        for i in 0..<self.points.count {
            self.points[i] = self.points[i] + transformation
        }
    }

    func matchingPoints(other: ScannerMap) -> Set<Point> {
        let lhs = Set<Point>(self.points)
        let rhs = Set<Point>(other.points)
        return lhs.intersection(rhs)
    }

    func findOrigin(toTransform: ScannerMap) -> Point? {
        var transforming = toTransform
        for i in 0..<self.points.count {
            for j in 0..<transforming.points.count {
                let transform = self.points[i].transformation(to: transforming.points[j])
                transforming.moveOrigin(to: transform)
                if self.matchingPoints(other: transforming).count >= 12 {
                    return transform
                }
            }
        }
        return nil
    }
}


var scanners : [ScannerMap] = []

while let input = readLine() {
    let comps = input.components(separatedBy: " ")
    if comps[0] == "---" {
        scanners.append(ScannerMap(id: Int(comps[2])!))
    }
}

if let movedPoint = scanners[0].findOrigin(toTransform: scanners[1]) {
    print("Found transformation: \(movedPoint)")
} else {
    print("Not found!")
}
