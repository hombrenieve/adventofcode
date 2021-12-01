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

func countWindow(_ v:[Int]) -> Int {
    if v.count < 4 {
        return 0
    }
    let w1 = v[0]+v[1]+v[2]
    let w2 = v[1]+v[2]+v[3]
    if w1 < w2 {
        return 1+countWindow(Array(v[1...]))
    }
    return countWindow(Array(v[1...]))
}

var v : [Int] = []

while let input = readLine() {
    if let dat = Int(input) {
        v.append(dat)
    } else {
        print("Can't read!")
    }
}

print("Increases: \(countWindow(v))")
