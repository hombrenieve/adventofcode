import Foundation

typealias Colony = [Int]

extension Colony {
    init(from: [String]) {
        self = [Int](repeating: 0, count: 9)
        for i in from {
            self[Int(i)!] += 1
        }
    }
    
    mutating func nextGeneration() {
        let spawns = self[0]
        for i in 1...8 {
            self[i-1] = self[i]
        }
        self[8] = spawns
        self[6] += spawns
    }
    
    func number() -> Int {
        var theCount = 0
        for i in 0...8 {
            theCount += self[i]
        }
        return theCount
    }
}

if let input = readLine() {
    var theColony = Colony(from: input.components(separatedBy: ","))
    let days = 256

    for day in 1...days {
        theColony.nextGeneration()
        print("After \(day) days there are \(theColony.number()) LanternFish")
    }
}
