import Foundation

func quicksort<T: Comparable>(_ a: [T]) -> [T] {
    guard a.count > 1 else { return a }

    let pivot = a[a.count/2]
    let less = a.filter { $0 < pivot }
    let equal = a.filter { $0 == pivot }
    let greater = a.filter { $0 > pivot }

    return quicksort(less) + equal + quicksort(greater)
}

typealias Counter = Dictionary<String, Int>

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

typealias Rules = Dictionary<String, RuleGraph>

extension Rules {
    func countPairs(_ prev: Counter) -> Counter {
        var counter: Counter = [:]
        for (k,_) in self {
            counter[k] = 0
        }
        for (_, r) in self {
            counter[r.left!.pair]! += prev[r.pair]!
            counter[r.right!.pair]! += prev[r.pair]!
        }
        return counter
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
}

func countLetters(fromPairs: Counter) -> Counter {
    var letterCounter: Counter = [:]
    for (k, c) in fromPairs {
        let key = Array(k)
        for i in 0...1 {
            let letter = String(key[i])
            if let exist = letterCounter[letter] {
                letterCounter[letter] = exist+c
            } else {
                letterCounter[letter] = c
            }
        }
    }
    return letterCounter
}

func calcResult(template: String, lettersFromPairs: Counter) -> Int {
    var count = lettersFromPairs
    count[String(template.first!)]! -= 1
    count[String(template.last!)]! -= 1
    count.forEach({ count[$0] = $1/2 })
    count[String(template.first!)]! += 1
    count[String(template.last!)]! += 1
    let ordered = quicksort(Array(count.values))
    return ordered.last! - ordered.first!
}



let polymerTemplate = Array(readLine()!)
_ = readLine() //empty one

var rules: Rules = [:]
var pairCounts: Counter = [:]

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
//Create counts
for (k,_) in rules {
    pairCounts[k] = 0
}

//Starting pair counts
for i in 0..<polymerTemplate.count-1 {
    let pair = String(polymerTemplate[i..<i+2])
    pairCounts[pair]! += 1
}

for _ in 1...steps {
    pairCounts = rules.countPairs(pairCounts)
}


//print("Pair count: \(pairCounts)")
let countFromPairs = countLetters(fromPairs: pairCounts)
//print("Count: \(countFromPairs)")
print("Result: \(calcResult(template: String(polymerTemplate), lettersFromPairs: countFromPairs))")