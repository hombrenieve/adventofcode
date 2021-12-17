import Foundation

struct Point {
    let x : Int
    let y : Int

    init(_ x: Int, _ y: Int) {
        self.x = x
        self.y = y
    }

    static func +(lhs: Point, rhs: Velocity) -> Point {
        return Point(
            lhs.x + rhs.x,
            lhs.y + rhs.y
        )
    }
}

struct Velocity {
    var x : Int
    var y : Int

    init(_ x: Int, _ y: Int) {
        self.x = x
        self.y = y
    }

    mutating func dec() {
        self.x = self.x == 0 ? 0 : self.x - 1
        self.y -= 1
    }
}

struct Trajectory {
    var v: Velocity
    var current: Point
    var maxY: Int = Int.min

    init(_ v: Velocity) {
        self.v = v
        self.current = Point(0,0)
    }

    mutating func next() {
        current = current + v
        maxY = max(self.maxY, current.y)
        v.dec()
    }

}

struct TargetArea {
    let minX: Int
    let maxX: Int
    let minY: Int
    let maxY: Int

    var initialX: [Int] {
        var initial: [Int] = []
        var i = 0
        var inc = 1
        while i <= self.maxX {
            if i > self.minX {
                initial.append(inc-1)
            }
            i += inc
            inc += 1
        }
        return initial
    }
    
    init(_ unorderedX: [Int], _ unorderedY: [Int]) {
        self.minX = unorderedX.min()!
        self.maxX = unorderedX.max()!
        self.minY = unorderedY.min()!
        self.maxY = unorderedY.max()!
    }
    
    func contained(_ point: Point) -> Bool {
        let (x, y) = (point.x, point.y)
        guard x >= self.minX,
            x <= self.maxX,
            y >= self.minY,
            y <= self.maxY else {
                return false
            }
        return true        
    }

    func missed(_ point: Point) -> Bool {
        let (x, y) = (point.x, point.y)
        if x > self.maxX || y < self.minY {
            return true
        }
        return false
    }

    func succeed(_ v: Velocity) -> (Bool, Int) {
        var trajectory = Trajectory(v)
        while !contained(trajectory.current) && !missed(trajectory.current) {
            trajectory.next()
        }
        return (contained(trajectory.current), trajectory.maxY)
    }

    func calculateMaxY() -> Int {
        var bestY = Int.min
        for y in 1...100 {
            for x in initialX {
                let v = Velocity(x, y)
                let (valid, calcY) = succeed(v)
                if valid {
                    bestY = max(bestY, calcY)
                    print("Success with \(v) -> \(calcY), Best: \(bestY)")
                }
            }
        }
        return bestY
    }
}

func createTargetArea(_ input: String) -> TargetArea {
    let comp = input.components(separatedBy: ": ")[1].components(separatedBy: ", ").map({ String(Array($0)[2..<$0.count]) }).flatMap({$0.components(separatedBy: "..")}).map({Int($0)!})
    return TargetArea(Array(comp[0...1]), Array(comp[2...3]))
}

let ta = createTargetArea(readLine()!)

print("Result: \(ta.calculateMaxY())")