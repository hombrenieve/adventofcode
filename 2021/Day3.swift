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

func getMostCommon(_ inString: String) -> String {
    let zeros = numberOfOccurrencesOf(string: "0", from: inString)
    let ones = numberOfOccurrencesOf(string: "1", from: inString)
    if ones < zeros {
        return "0"
    }
    return "1"
}

func getLeastCommon(_ inString: String) -> String {
    let zeros = numberOfOccurrencesOf(string: "0", from: inString)
    let ones = numberOfOccurrencesOf(string: "1", from: inString)
    if ones < zeros {
        return "1"
    }
    return "0"
}

func findPattern(data: [String], pattern: String, comparator: (String)->String) -> Int {
    if data.count == 1 {
        return Int(data[0], radix: 2)!
    }
    let column = pattern.count
    let columnString = stringFrom(column: column, of: data)
    let newPattern = pattern + comparator(columnString)
    let newArray = data.filter({$0.hasPrefix(newPattern)})
    return findPattern(data: newArray, pattern: newPattern, comparator: comparator)
}

func findOxygenGeneratorRating(array: [String]) -> Int {
    return findPattern(data: array, pattern: "", comparator: getMostCommon)
}

func findCO2ScrubberRating(array: [String]) -> Int {
    return findPattern(data: array, pattern: "", comparator: getLeastCommon)
}


var input : [String] = []
while let line = readLine() {
    input.append(line)
}

let oxygen = findOxygenGeneratorRating(array: input)
let co2 = findCO2ScrubberRating(array: input)

print("Gener: \(oxygen)")
print("CO2: \(co2)")
print("Res: \(oxygen*co2)")
