import Foundation

func divideAndRoundUp(_ number: Int) -> Int {
    return Int(ceil(Double(number) / 2.0))
}

func divideAndRoundDown(_ number: Int) -> Int {
    return Int(floor(Double(number) / 2.0))
}

class SFN {
    var left: SFN? = nil
    var right: SFN? = nil
    var value: Int? = nil

    init(_ v: Int) {
        self.value = v
    }

    init(_ x: SFN, _ y: SFN) {
        self.left = x
        self.right = y
    }

    func getValue() -> Int? {
        return value
    }

    func description() -> String {
        if let v = getValue() {
            return String(v)
        }
        var str = "["
        str += self.left!.description()
        str += ","
        str += self.right!.description()
        str += "]"
        return str
    }

    static func +(lhs: SFN, rhs: SFN) -> SFN {
        let sfn = SFN(lhs.copy(), rhs.copy())
        sfn.reduce()
        return sfn
    }

    func findLeafs(_ curr: Int) -> [LEAF] {
        if let _ = self.getValue() {
            return [LEAF(curr, self)]
        }
        if let l = self.left {
            if let r = self.right {
                guard let _ = l.getValue(),
                    let _ = r.getValue()
                else {
                    return l.findLeafs(curr+1) + r.findLeafs(curr+1)
                }                
                return [LEAF(curr, self)]
            } else {
                return l.findLeafs(curr+1)
            }
        } else {
            if let r = self.right {
                return r.findLeafs(curr+1)
            }
        }
        return []
    }

    func findBigNumber() -> SFN? {
        if let v = self.getValue() {
            if v > 9 {
                return self
            }
            return nil
        }
        if let l = self.left {
            if let found = l.findBigNumber() {
                return found
            }
        }
        if let r = self.right {
            if let found = r.findBigNumber() {
                return found
            }
        }
        return nil
    }

    func explode() -> Bool {
        let leafs = self.findLeafs(0)
        guard let iToExplode = leafs.firstIndex(where: { 
            $0.level >= 4 &&
            $0.leaf.getValue() == nil &&
            $0.leaf.left != nil &&
            $0.leaf.left!.getValue() != nil &&
            $0.leaf.right != nil &&
            $0.leaf.right!.getValue() != nil
            })
            else {
                return false
            }
        let leftNumber = leafs[iToExplode].leaf.left!.getValue()!
        let rightNumber = leafs[iToExplode].leaf.right!.getValue()!
        leafs[iToExplode].leaf.left = nil
        leafs[iToExplode].leaf.right = nil
        leafs[iToExplode].leaf.value = 0
        if iToExplode > 0 {
            leafs[iToExplode-1].leaf.addRight(number: leftNumber)
        }
        if iToExplode < leafs.count-1 {
            leafs[iToExplode+1].leaf.addLeft(number: rightNumber)
        }
        return true
    }

    func addLeft(number: Int) {
        if let v = self.getValue() {
            self.value = v+number
        } else if let l = self.left {
            l.addLeft(number: number)
        }
    }

    func addRight(number: Int) {
        if let v = self.getValue() {
            self.value = v+number
        } else if let r = self.right {
            r.addRight(number: number)
        }
    }

    func split() -> Bool {
        guard let toSplit = self.findBigNumber() 
            else {
                return false
            }
        let v = toSplit.getValue()!
        toSplit.value = nil
        toSplit.left = SFN(divideAndRoundDown(v))
        toSplit.right = SFN(divideAndRoundUp(v))
        return true
    }

    func reduce() {
        while explode() || split() {
        }
    }

    func magnitude() -> Int {
        if let v = self.getValue() {
            return v
        }
        var mag = 0
        if let l = self.left {
            mag += 3*l.magnitude()
        }
        if let r = self.right {
            mag += 2*r.magnitude()
        }
        return mag
    }


    func copy() -> SFN {
        if let v = getValue() {
            return SFN(v)
        }
        var lc: SFN? = nil
        var rc: SFN? = nil
        if let l = self.left {
            lc = l.copy()
        }
        if let r = self.right {
            rc = r.copy()
        }
        return SFN(lc!, rc!)
    }
}

struct LEAF {
    let level: Int
    let leaf: SFN

    init(_ lev: Int, _ leaf: SFN) {
        self.level = lev
        self.leaf = leaf
    }
}

func parseInput(_ input: [Character], _ curr: Int) -> (SFN?, Int) {
    if input.count == curr {
        return (nil, curr)
    }
    switch(input[curr]) {
        case "[":
            let (left, next) = parseInput(input, curr+1)
            let (right, next2) = parseInput(input, next+1)
            return (SFN(left!, right!), next2)
        case "]": //Ignored
            fallthrough
        case ",":
            var ignored = curr
            while input[ignored] == "]" || input[ignored] == "," {
                ignored += 1
            }
            return parseInput(input, ignored)
        default: //nr
            return (SFN(input[curr].wholeNumberValue!), curr+1)         
    }
}

var numbers: [SFN] = []

while let line = readLine() {
    numbers.append(parseInput(Array(line), 0).0!)
}

var largest = 0
for i in 0..<numbers.count {
    for j in 0..<numbers.count {
        guard i != j else { continue }
        let sum = numbers[i]+numbers[j]
        largest = max(largest, sum.magnitude())
    }
}

print("Magnitude: \(largest)")