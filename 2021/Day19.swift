import Foundation

struct Point: Hashable, Comparable {
    let x : Int
    let y : Int
    let z : Int
    init(_ x: Int, _ y: Int, _ z: Int) {
        self.x = x
        self.y = y
        self.z = z
    }
    func hash(into hasher: inout Hasher) {
        hasher.combine(x)
        hasher.combine(y)
        hasher.combine(z)
    }
    static func ==(lhs: Point, rhs: Point) -> Bool {
        return lhs.x == rhs.x && lhs.y == rhs.y && lhs.z == rhs.z
    }
    static func < (lhs: Point, rhs: Point) -> Bool {
        return lhs.x < rhs.x && lhs.y < rhs.y && lhs.z < rhs.z
    }
}

struct ScannerMap {
    let id: Int
    let points: [Point]

    init(id: Int) {
        self.id = id
        var points: [Point] = []
        while let input = readLine() {
            if input == "" {
                break
            }
            let comps = input.components(separatedBy: ",")
            points.append(Point(Int(comps[0])!, Int(comps[1])!, Int(comps[2])!))
        }
        self.points = points
    }
}


var scanners : [ScannerMap] = []

while let input = readLine() {
    let comps = input.components(separatedBy: " ")
    if comps[0] == "---" {
        scanners.append(ScannerMap(id: Int(comps[2])!))
    }
}