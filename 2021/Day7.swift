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

func costOfFuel(from: Int, to: Int) -> Int {
    let distance = abs(from-to)
    return (distance)*(distance+1)/2
}

extension CrabSwarm {
    init(from: [String]) {
        self = quicksort(from.map { Int($0)! })
    }
    func costOfMoving(to: Int) -> Int {
        return self.map({ costOfFuel(from: $0, to: to) }).reduce(0, +)
    }
    func findBest() -> (Int, Int) {
        var best = (-1, Int.max)
        for i in 0...self[self.count-1] {
            let c = self.costOfMoving(to: i)
            print("Cost of \(i) if \(c)")
            if c < best.1 {
                best = (i, c)
            }
        }
        return best
    }
}

if let input = readLine() {
    let crabs = CrabSwarm(from: input.components(separatedBy: ","))
    print("Best Hit: \(crabs.findBest())")
}