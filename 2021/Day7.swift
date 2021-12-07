import Foundation

func quicksort<T: Comparable>(_ a: [T]) -> [T] {
    guard a.count > 1 else { return a }

    let pivot = a[a.count/2]
    let less = a.filter { $0 < pivot }
    let equal = a.filter { $0 == pivot }
    let greater = a.filter { $0 > pivot }

    return quicksort(less) + equal + quicksort(greater)
}

typealias CrabSwarm = [Int]

extension CrabSwarm {
    init(from: [String]) {
        self = quicksort(from.map { Int($0)! })
    }
    func costOfMoving(to: Int) -> Int {
        return self.map({ abs($0 - to) }).reduce(0, +)
    }
    func findBest() -> (Int, Int) {
        var best = (-1, Int.max)
        var last = -1
        for i in 0...self.count-1 {
            if self[i] != last {
                let c = self.costOfMoving(to: self[i])
                if c < best.1 {
                    best = (self[i], c)
                }
                last = self[i]
            }
        }
        return best
    }
}

if let input = readLine() {
    let crabs = CrabSwarm(from: input.components(separatedBy: ","))
    print("Best Hit: \(crabs.findBest())")
}