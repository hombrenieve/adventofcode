import Foundation

struct Position {
    var horizontal: Int = 0
    var depth: Int = 0
    var aim: Int = 0

    mutating func newPos(cmd: String, amt: Int) {
        switch(cmd) {
        case "forward":
            horizontal += amt
            depth += aim*amt
        case "down":
            aim += amt
        case "up":
            aim -= amt
        default:
            print("NOWAY")
        }
    }
}

var pos = Position()
while let input = readLine() {
    let separated = input.components(separatedBy: " ")
    let cmd = separated[0]
    let amt = Int(separated[1])!
    pos.newPos(cmd: cmd, amt: amt)
}

print("Result is: \(pos.depth*pos.horizontal)")
