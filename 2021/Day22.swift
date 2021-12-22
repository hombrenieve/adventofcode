import Foundation

struct Cubit: Hashable, Comparable {
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
    static func ==(lhs: Cubit, rhs: Cubit) -> Bool {
        return lhs.x == rhs.x && lhs.y == rhs.y && lhs.z == rhs.z
    }
    static func < (lhs: Cubit, rhs: Cubit) -> Bool {
        return lhs.x < rhs.x && lhs.y < rhs.y && lhs.z < rhs.z
    }
}

struct Cube {
    let points: [Cubit] //must be exactly 8
    let xlimits: [Int]
    let ylimits: [Int]
    let zlimits: [Int]
    var state: Bool = false
    var volume: Int {
        let xlength = abs(xlimits[1]-xlimits[0])+1
        let ylength = abs(ylimits[1]-ylimits[0])+1
        let zlength = abs(zlimits[1]-zlimits[0])+1
        return xlength*ylength*zlength
    }

    init(xs: [Int], ys: [Int], zs: [Int]) {
        var cubits = [Cubit]()
        for i in xs {
            for j in ys {
                for k in zs {
                    cubits.append(Cubit(i,j,k))
                }
            }
        }
        self.points = cubits
        self.xlimits = xs
        self.ylimits = ys
        self.zlimits = zs
    }

    init(state: Bool, xs: [Int], ys: [Int], zs: [Int]) {
        self.init(xs: xs, ys: ys, zs: zs)
        self.state = state
    }

    func contains(point: Cubit) -> Bool {
        return self.xlimits[0] < point.x && point.x < self.xlimits[1] &&
            self.ylimits[0] < point.y && point.y < self.ylimits[1] &&
            self.zlimits[0] < point.z && point.z < self.zlimits[1]
    }

    static func ==(lhs: Cube, rhs: Cube) -> Bool {
        return lhs.xlimits == rhs.xlimits && lhs.ylimits == rhs.ylimits && lhs.zlimits == rhs.zlimits
    }

    func contains(other: Cube) -> Bool {
        for element in other.points {
            if !self.contains(point: element) {
                return false
            }
        }
        return true
    }

    func intersect(other: Cube) -> Cube? {
        if other.contains(other: self) {
            return Cube(state: other.state, xs: self.xlimits, ys: self.ylimits, zs: self.zlimits)
        }
        if self.contains(other: other) {
            return other
        }
        if self == other {
            return other
        }
        var intersection = false
        for p in other.points {
            if self.contains(point: p) {
                intersection = true
                break              
            }
        }
        for p in self.points {
            if other.contains(point: p) {
                intersection = true
                break              
            }
        }
        if !intersection {
            return nil
        }
        let xlim = (self.xlimits + other.xlimits).sorted()
        let ylim = (self.ylimits + other.ylimits).sorted()
        let zlim = (self.zlimits + other.zlimits).sorted()
        return Cube(state: other.state, xs: Array(xlim[1...2]), ys: Array(ylim[1...2]), zs: Array(zlim[1...2]))
    }
}

struct Reactor {
    var areas: [Cube] = []
    var intersections: [Cube] = []
    static let region = Cube(xs: [-50, 50], ys: [-50, 50], zs: [-50, 50])

    private static func inRegion(cube: Cube) -> Bool {
        return Reactor.region.contains(other: cube)
    }

    mutating func apply(step: Bool, xs: [Int], ys: [Int], zs: [Int]) {
        let cube = Cube(state: step, xs: xs, ys: ys, zs: zs)
        if Reactor.inRegion(cube: cube) {
            for c in areas {
                if let inter = c.intersect(other: cube) {
                    intersections.append(inter)
                }
            }
            if step {
                areas.append(cube)
            }
        }
    }

    func onlineCount() -> Int {
        let onVolume = areas.filter({ $0.state }).map({$0.volume}).reduce(0,+)
        let intersectOnVolume = intersections.filter({ $0.state }).map({$0.volume}).reduce(0,+)
        let intersectOffVolume = intersections.filter({ !$0.state }).map({$0.volume}).reduce(0,+)
        return onVolume-intersectOnVolume-intersectOffVolume
    }

}

func createCommand(from: String) -> (Bool, [Int], [Int], [Int]) {
    let step = from.components(separatedBy: " ")[0] == "on"
    let comps = from.components(separatedBy: " ")[1].components(separatedBy: ",").map({ String(Array($0)[2..<$0.count]) }).flatMap({$0.components(separatedBy: "..")}).map({Int($0)!})
    return (step, Array(comps[0...1]), Array(comps[2...3]), Array(comps[4...5]))
}

var reactor = Reactor()

while let input = readLine() {
    let (step, xs, ys, zs) = createCommand(from: input)
    reactor.apply(step: step, xs: xs, ys: ys, zs: zs)
    print("Areas: \(reactor.areas.map({ ($0.volume, $0.state) })) Intersect: \(reactor.intersections.map({ ($0.volume, $0.state) }))")
}

print("After all steps there are: \(reactor.onlineCount())")