import Foundation

func quicksort<T: Comparable>(_ a: [T]) -> [T] {
    guard a.count > 1 else { return a }

    let pivot = a[a.count/2]
    let less = a.filter { $0 < pivot }
    let equal = a.filter { $0 == pivot }
    let greater = a.filter { $0 > pivot }

    return quicksort(less) + equal + quicksort(greater)
}

typealias Counter = Dictionary<Character, Int>
var rules: Dictionary<String, RuleGraph> = [:]

extension Counter {
    func merge(_ rhs: Counter) -> Counter {
        var result = self
        for (k, v) in rhs {
            if let exist = self[k] {
                result[k] = exist + v
            } else {
                result[k] = v
            }
        }
        return result
    }
}

class RuleGraph: Equatable {
    let pair: String
    let insert: String
    var left: RuleGraph?
    var right: RuleGraph?

    init(_ pair: String, _ insert: String) {
        self.pair = pair
        self.insert = insert
    }

    static func ==(lhs: RuleGraph, rhs: RuleGraph) -> Bool {
        return lhs.pair == rhs.pair
    }
    
    func generate() -> (String, String) {
        return (String(pair.first!)+insert, insert+String(pair.last!))
    }

    private func countLocalLetters() -> Counter {
        var result: Counter = [:]
        result[insert.first!] = 1
        return result
    }

    func countLetters(curr: Int, max: Int) -> Counter {
        print("Current level: \(curr)")
        if curr == max {
            return countLocalLetters()
        }
        let leftCount = self.left!.countLetters(curr: curr+1, max: max)
        let rightCount = self.right!.countLetters(curr: curr+1, max: max)
        return countLocalLetters().merge(leftCount).merge(rightCount)
    }

}

func calculateResult(_ counts: Counter) -> Int {
    let counted = quicksort(Array(counts.values))
    return counted.last! - counted.first!
}

let polymerTemplate = Array(readLine()!)
_ = readLine() //empty one

while let input = readLine() {
    let split = input.components(separatedBy: " -> ")
    rules[split[0]] = RuleGraph(split[0], split[1])
}

for (_, rule) in rules {
    let names = rule.generate()
    rule.left = rules[names.0]!
    rule.right = rules[names.1]!
}


let steps = 40
var elementCount: Counter = [:]
for c in polymerTemplate {
    if let _ = elementCount[c] {
        elementCount[c]! += 1
    } else {
        elementCount[c] = 1
    }
}
for i in 0..<polymerTemplate.count-1 {
    //print("Count is: \(elementCount)")
    let pair = String(polymerTemplate[i..<i+2])
    //print("Processing \(pair)")
    let head = rules[pair]!
    elementCount = elementCount.merge(head.countLetters(curr: 1, max: steps))
}

print("Result: \(calculateResult(elementCount))")