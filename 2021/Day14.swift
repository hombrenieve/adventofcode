import Foundation

func quicksort<T: Comparable>(_ a: [T]) -> [T] {
    guard a.count > 1 else { return a }

    let pivot = a[a.count/2]
    let less = a.filter { $0 < pivot }
    let equal = a.filter { $0 == pivot }
    let greater = a.filter { $0 > pivot }

    return quicksort(less) + equal + quicksort(greater)
}

class PolymerElement {
    let name: Character
    var next: PolymerElement?
    var prev: PolymerElement?
    var mirror: PolymerElement?

    init(_ name: Character) {
        self.name = name
    }
}

func polymerToString(_ data: PolymerElement) -> String {
    var result = "" + String(data.name)
    var cursor = data
    while let next = cursor.next {
        result.append(String(next.name))
        cursor = next
    }
    return result
}

struct Polymer {
    var pTree: PolymerElement
    var rules: Dictionary<String, Character> = [:]

    init(_ initial: String) {
        let chars = Array(initial)
        pTree = PolymerElement(chars[0])
        pTree.next = PolymerElement(chars[1])
        var current = pTree.next!
        current.prev = pTree
        for i in 2..<chars.count {
            let prev = current
            current.next = PolymerElement(chars[i])
            current = current.next!
            current.prev = prev
        }
    }

    mutating func addRule(pair: String, insert: Character) {
        rules[pair] = insert
    }

    mutating func buildMirror() -> PolymerElement {
        var cursor = pTree
        let mirror = PolymerElement(pTree.name)
        var prevMirror = mirror
        pTree.mirror = mirror
        while let next = cursor.next {
            let nextMirror = PolymerElement(next.name)
            nextMirror.prev = prevMirror
            prevMirror.next = nextMirror
            cursor = next
            cursor.mirror = nextMirror
            prevMirror = nextMirror            
        }
        return mirror

    }

    mutating func applyRules() {
        let mirror = buildMirror()
        var cursor = pTree
        while let next = cursor.next {
            let pair = String(cursor.name)+String(next.name)
            if let insert = rules[pair] {
                let newElement = PolymerElement(insert)
                cursor.mirror!.next = newElement
                newElement.prev = cursor.mirror
                next.mirror!.prev = newElement
                newElement.next = next.mirror
            }
            cursor = next
        }
        pTree = mirror
    }

    func calculateResult() -> Int {
        var elementCount: Dictionary<Character,Int> = [:]
        var cursor = pTree
        elementCount[cursor.name] = 1
        while let next = cursor.next {
            if let currentCount = elementCount[next.name] {
                elementCount[next.name] = currentCount+1
            } else {
                elementCount[next.name] = 1
            }
            cursor = next
        }
        let counts = quicksort(Array(elementCount.values))
        return counts.last! - counts.first!
    }
}

var polymer = Polymer(readLine()!)
_ = readLine() //empty one

while let input = readLine() {
    let split = input.components(separatedBy: " -> ")
    polymer.addRule(pair: split[0], insert: split[1].first!)
}

print("Read: \(polymerToString(polymer.pTree))")

let count = 10
for i in 1...count {
    print("Step \(i)/\(count)")
    polymer.applyRules()
}

print("Result: \(polymer.calculateResult())")
