import Foundation

func neededFuel(ofModule: Int) -> Int {
    if ofModule < 6 {
        return 0
    }
    let mass = Int(floor(Double(ofModule) / 3.0)-2)
    return mass+neededFuel(ofModule: mass)
}

var sum : Int = 0
while let input = readLine() {
    if let dat = Int(input) {
        sum += neededFuel(ofModule: dat)
    }
}
print("Sum: \(sum)")