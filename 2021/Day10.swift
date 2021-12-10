import Foundation

func quicksort<T: Comparable>(_ a: [T]) -> [T] {
    guard a.count > 1 else { return a }

    let pivot = a[a.count/2]
    let less = a.filter { $0 < pivot }
    let equal = a.filter { $0 == pivot }
    let greater = a.filter { $0 > pivot }

    return quicksort(less) + equal + quicksort(greater)
}

struct SyntaxChecker {
    private let chunks : [String]

    private static let score = [
        ")": 3,
        "]": 57,
        "}": 1197,
        ">": 25137
    ]
    private static let completionScore = [
        ")": 1,
        "]": 2,
        "}": 3,
        ">": 4
    ]
    private static let initials = "([{<"

    init(_ theNav: [String]) {
        chunks = theNav
    }

    private static func corruptedCharIn(line: String) -> Character? {
        var stack: [Character] = []
        for delimiter in line {
            if initials.contains(delimiter) {
                stack.append(delimiter)
            } else {
                let initial = stack.last
                stack.removeLast()
                switch (initial, delimiter) {
                    case ("<", ">"), ("[", "]"), ("(", ")"), ("{", "}"):
                        continue
                    default:
                        return delimiter
                }
            }
        }
        return nil
    }

    private static func completionStringFor(line: String) -> String {
        var stack: [Character] = []
        var completion = ""
        for delimiter in line {
            if initials.contains(delimiter) {
                stack.append(delimiter)
            } else {
                let initial = stack.last
                stack.removeLast()
                switch (initial, delimiter) {
                    case ("<", ">"), ("[", "]"), ("(", ")"), ("{", "}"):
                        continue
                    default:
                        print("Is Corrupted!")
                        return ""
                }
            }
        }
        for initial in stack.reversed() {
            switch initial {
                case "(":
                    completion += ")"
                case "[":
                    completion += "]"
                case "{":
                    completion += "}"
                case "<":
                    completion += ">"
                default:
                    print("Unrecognized character!")
                    return ""
            }
        }
        return completion
    }

    func scoreCorrupted() -> Int {
        var scored = 0
        for line in chunks {
            if let delimiter = SyntaxChecker.corruptedCharIn(line: line) {
                scored += SyntaxChecker.score[String(delimiter)]!
            }
        }
        return scored
    }

    func scoreCompletion() -> Int {
        let uncorrupted = chunks.filter({SyntaxChecker.corruptedCharIn(line: $0) == nil})
        let completions = uncorrupted.map({SyntaxChecker.completionStringFor(line: $0)})
        var scores: [Int] = []
        print("Completions: \(completions)")
        for line in completions {
            var partialScore = 0
            for del in line {
                partialScore *= 5
                partialScore += SyntaxChecker.completionScore[String(del)]!
            }
            scores.append(partialScore)
            print("Score of \(line) is \(partialScore)")
        }
        return quicksort(scores)[scores.count/2]
    }


}

var navigation : [String] = []
while let input = readLine() {
    navigation.append(input)
}

let sx = SyntaxChecker(navigation)

print("Score: \(sx.scoreCompletion())")