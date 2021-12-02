import Foundation

struct Position {
    var horizontal: Int = 0
    var depth: Int = 0

    mutating func newPos(cmd: String, amt: Int) {
        switch(cmd) {
        case "forward":
            horizontal += amt
        case "down":
            depth += amt
        case "up":
            depth -= amt
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
