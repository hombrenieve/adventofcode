import Foundation

struct LanternFish {
    static let INITIAL = 8
    static let RESET = 6
    var timer: Int
    
    init(_ starting: Int = LanternFish.INITIAL) {
        self.timer = starting
    }
    
    mutating func nextGeneration() -> Bool {
        //Return true if reaches 0
        self.timer -= 1
        if self.timer == -1 {
            self.timer = LanternFish.RESET
            return true
        }
        return false
    }
    
}

typealias Colony = [LanternFish]

extension Colony {
    init(from: [String]) {
        self = []
        for i in from {
            self.append(LanternFish(Int(i)!))
        }
    }
    mutating func generate(spawns: Int) {
        for _ in 1...spawns {
            self.append(LanternFish())
        }
    }
    mutating func nextGeneration() {
        var spawns = 0
        for i in 0...self.count-1 {
            if self[i].nextGeneration() {
                spawns += 1
            }
        }
        if spawns > 0 {
            generate(spawns: spawns)
        }
    }
}

if let input = readLine() {
    var theColony = Colony(from: input.components(separatedBy: ","))
    let days = 80

    for day in 1...days {
        theColony.nextGeneration()
        print("After \(day) days there are \(theColony.count) LanternFish")
    }
}
