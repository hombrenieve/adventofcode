import Foundation

var sum : Int = 0
while let input = readLine() {
    if let dat = Double(input) {
        sum += Int(floor(dat / 3.0)-2)
    }
}
print("Sum: \(sum)")