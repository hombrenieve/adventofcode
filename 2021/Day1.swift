import Foundation

func countIncrease(v: [Int]) -> Int {
    var prev = v[0]
    var increases = 0
    for i in 1...v.count-1 {
        if v[i] > prev {
            increases+=1
        }
        prev = v[i]
    }
    return increases
}

var v : [Int] = []

while let input = readLine() {
    if let dat = Int(input) {
        v.append(dat)
    } else {
        print("Can't read!")
    }
}

print("Increases: \(countIncrease(v: v))")
