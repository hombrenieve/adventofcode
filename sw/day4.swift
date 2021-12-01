public func checkPassword(number: Int)-> Bool {
    return checkSize(number)  && checkAdjacent(number) && checkIncrease(number)
}

func checkSize(_ number: Int)-> Bool {
    return String(number).count == 6
}

func checkAdjacent(_ number: Int)-> Bool {
    let stNumber = String(number).compactMap{ $0.wholeNumberValue }
    return (stNumber[0] == stNumber[1] && stNumber[1] != stNumber[2]) || 
        (stNumber[0] != stNumber[1] && stNumber[1] == stNumber[2] && stNumber[2] != stNumber[3]) ||
        (stNumber[3] != stNumber[4] && stNumber[4] == stNumber[5]) ||
        (stNumber[1] != stNumber[2] && stNumber[2] == stNumber[3] && stNumber[3] != stNumber[4]) ||
        (stNumber[2] != stNumber[3] && stNumber[3] == stNumber[4] && stNumber[4] != stNumber[5])
}

func checkIncrease(_ number: Int)-> Bool {
    if String(number).count == 1 {
        return true
    }
    let curr = number % 10
    let next = (number/10) % 10
    if curr < next {
        return false
    }
    return checkIncrease(number / 10)
}

public func calculateRange(low: Int, high: Int)->Int {
    var passwords = 0
    for number in low...high {
        if checkPassword(number: number) {
            passwords += 1
        }
    }
    return passwords
}

//let res = checkPassword(number: 222222)
let res = calculateRange(low: 367479, high: 893698)
print("Res: \(res)")