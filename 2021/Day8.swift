import Foundation

extension String {
    func removingLeadingSpaces() -> String {
        guard let index = firstIndex(where: { !CharacterSet(charactersIn: String($0)).isSubset(of: .whitespaces) }) else {
            return self
        }
        return String(self[index...])
    }
}

func contains(chars: String, on: String) -> Bool {
    for i in chars {
        if on.contains(i) == false {
            return false
        }
    }
    return true
}

func containsExactly(chars: String, on: String) -> Bool {
    if chars.count != on.count {
        return false
    }
    for i in chars {
        if on.contains(i) == false {
            return false
        }
    }
    return true
}

func complementSix(digit: String) -> String {
    for i in "abcdefg" {
        if digit.contains(i) == false {
            return String(i)
        }
    }
    return ""
}

struct SevenSeg {
    var digits = [String](repeating: "", count: 10)
    
    init(_ pattern: [String]) {
        deduceOne(pattern)
        deduceSeven(pattern)
        deduceEight(pattern)
        deduceFour(pattern)
        deduceZeroAndNineAndSix(pattern)
        deduceTwoThreeFive(pattern)
    }
    
    //Executed in order
    mutating func deduceOne(_ pattern: [String]) {
        digits[1] = pattern.filter({$0.count == 2})[0]
    }
    mutating func deduceSeven(_ pattern: [String]) {
        digits[7] = pattern.filter({$0.count == 3})[0]
    }
    mutating func deduceEight(_ pattern: [String]) {
        digits[8] = pattern.filter({$0.count == 7})[0]
    }
    mutating func deduceFour(_ pattern: [String]) {
        digits[4] = pattern.filter({$0.count == 4})[0]
    }
    mutating func deduceZeroAndNineAndSix(_ pattern: [String]) {
        let temp = pattern.filter({$0.count == 6})
        digits[9] = temp.filter({contains(chars: digits[4], on: $0)})[0]
        let tempNoNine = temp.filter({$0 != digits[9]})
        digits[0] = tempNoNine.filter({contains(chars: digits[1], on: $0)})[0]
        digits[6] = tempNoNine.filter({$0 != digits[0]})[0]
    }
    mutating func deduceTwoThreeFive(_ pattern: [String]) {
        let temp = pattern.filter({$0.count == 5})
        digits[3] = temp.filter({contains(chars: digits[1], on: $0)})[0]
        let tempNoThree = temp.filter({$0 != digits[3]})
        let compSix = complementSix(digit: digits[6])
        digits[2] = tempNoThree.filter({contains(chars: compSix, on: $0)})[0]
        digits[5] = tempNoThree.filter({$0 != digits[2]})[0]
    }
    
    func index(of: String) -> Int {
        return digits.firstIndex(where: { containsExactly(chars: of, on: $0)})!
    }
    
    func decode(code: [String]) -> Int {
        var decoded = ""
        //print("To decode: \(code)")
        for digit in code {
            //print("Looking for \(digit)")
            let element = index(of: digit)
            decoded.append(String(element))
        }
        return Int(decoded)!
    }
}



var instances = 0

while let input = readLine() {
    let pattern = input.components(separatedBy: "|")[0].components(separatedBy: " ")
    let output = input.components(separatedBy: "|")[1].removingLeadingSpaces().components(separatedBy: " ")
    
    var display = SevenSeg(pattern)
    print("So far: \(display)")
    
    let decoded = display.decode(code: output)
    
    print("Decoded: \(decoded)")
    
    instances += decoded
    
}

print("Total instances: \(instances)")

