import Foundation

func numberOfOccurrencesOf(string: String, from: String) -> Int {
    return from.components(separatedBy:string).count - 1
}

func stringFrom(column: Int, of: [String]) -> String {
    var res = ""
    for i in of {
        let p = Array(i)
        res += String(p[column])
        
    }
    return res
}

func getMostAndLessCommon(inString: String) -> (String, String) {
    let zeros = numberOfOccurrencesOf(string: "0", from: inString)
    let ones = numberOfOccurrencesOf(string: "1", from: inString)
    if zeros < ones {
        return ("1", "0")
    }
    return ("0", "1")
}

func calculateValues(array: [String]) -> (Int, Int) {
    var gamma = ""
    var epsilon = ""
    let wordSize = array[0].count
    for index in 0..<wordSize {
        let column = stringFrom(column: index, of: array)
        let (compGamma, compEpsilon) = getMostAndLessCommon(inString: column)
        gamma += compGamma
        epsilon += compEpsilon
    }
    return (Int(gamma, radix:2)!, Int(epsilon, radix:2)!)
}


var input : [String] = []
while let line = readLine() {
    input.append(line)
}

let (gamma, epsilon) = calculateValues(array: input)
print("gamma: \(gamma), epsilon: \(epsilon)")
print("res: \(gamma*epsilon)")
