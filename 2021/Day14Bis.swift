import Foundation


var rules: Dictionary<String, String> = [:]

class RuleGraph {
    let head: RuleGraph?
    let pair: String
    var left: RuleGraph?
    var right: RuleGraph?

    init(_ pair: String, head: RuleGraph?) {
        self.pair = pair
        self.head = head
        let names = self.generate()
        if head == nil {
            self.left = RuleGraph(names.0, head: self)
            self.right = RuleGraph(names.1, head: self)
        } else {
            if let left = head!.find(names.0) {
                self.left = left
            } else {
                self.left = RuleGraph(names.0, head: head)
            }
            if let right = head!.find(names.1) {
                self.right = right
            } else {
                self.right = RuleGraph(names.1, head: head)
            }
        }
    }
    
    private func generate() -> (String, String) {
        let ins = rules[pair]!
        return (String(pair.first!)+ins, ins+String(pair.last!))
    }

    func find(_ elem: String) -> RuleGraph? {
        if elem == self.pair {
            return self
        }
        if let left = self.left {
            if let found = left.find(elem) {
                return found
            }
        }
        if let right = self.right {
            if let found = right.find(elem) {
                return found
            }
        }
        return nil
    }
}

let polymerTemplate = Array(readLine()!)
_ = readLine() //empty one

while let input = readLine() {
    let split = input.components(separatedBy: " -> ")
    rules[split[0]] = split[1]
}

var ruleGraphs: [RuleGraph] = []

for i in 0..<polymerTemplate.count {
    ruleGraphs.append(RuleGraph(String(polymerTemplate[i..<i+2]), head: nil))
}