import Foundation

struct Point: Hashable, Equatable {
    let x : Int
    let y : Int

    init(_ x: Int, _ y: Int) {
        self.x = x
        self.y = y
    }

    func hash(into hasher: inout Hasher) {
        hasher.combine(x)
        hasher.combine(y)
    }
    static func ==(lhs: Point, rhs: Point) -> Bool {
        return lhs.x == rhs.x && lhs.y == rhs.y
    }
}

struct OrigamiPaper {
    var dots: Set<Point> = []

    init(_ input: [String]) {
        for line in input {
            let pointArray = line.components(separatedBy: ",").map({Int($0)!})
            dots.insert(Point(pointArray[0], pointArray[1]))
        }
    }

    mutating func foldY(_ axis: Int) {
        let filtered = dots.filter({$0.y > axis})
        let transformed = filtered.map({Point($0.x, axis-($0.y-axis))})
        dots.subtract(filtered)
        dots.formUnion(transformed)
    }
    mutating func foldX(_ axis: Int) {
        let filtered = dots.filter({$0.x > axis})
        let transformed = filtered.map({Point(axis-($0.x-axis), $0.y)})
        dots.subtract(filtered)
        dots.formUnion(transformed)
    }

    mutating func apply(instruction: (String, Int)) {
        switch(instruction) {
            case ("x", _):
            foldX(instruction.1)
            case ("y", _):
            foldY(instruction.1)
            default:
            print("Unrecognized instruction")
        }
    }


    func printer() {
        var maxX = 0
        var maxY = 0
        for i in dots {
            maxX = max(i.x, maxX)
            maxY = max(i.y, maxY)
        }
        var plotter = [[String]](repeating: [String](repeating: " ", count: maxX+1), count: maxY+1)
        for value in dots {
            plotter[value.y][value.x] = "#"
        }
        for line in plotter {
            print("\(line.reduce("", +))")
        }
    }

}



var dots: [String] = []
var instructions: [(String, Int)] = []
while let line = readLine() {
    if line == "" {
        break
    }
    dots.append(line)
}
while let line = readLine() {
    let comp = line.components(separatedBy: " ")[2].components(separatedBy: "=")
    instructions.append((comp[0], Int(comp[1])!))
}

var paper = OrigamiPaper(dots)

for ins in instructions {
    paper.apply(instruction: ins)
}

paper.printer()