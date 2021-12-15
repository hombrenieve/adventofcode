import Foundation

var caveMap: [[Int]] = []
typealias Position = (Int, Int)
var waysFound: [Int] = []

func findWays(curr: Position, accum: Int) {
    let (x,y) = curr
    //check prune
    if waysFound.count > 0 {
        if accum > waysFound.min()! {
            return
        }
    }
    guard x < caveMap[0].count,
        y < caveMap.count
         else {
            return
        }
    if x == caveMap[0].count-1 && y == caveMap.count-1 {
        waysFound.append(accum)
        return
    }
    findWays(curr: (x+1, y), accum: accum+caveMap[y][x])
    findWays(curr: (x, y+1), accum: accum+caveMap[y][x])
}


while let input = readLine() {
    caveMap.append(input.map({$0.wholeNumberValue!}))
}

findWays(curr: (0, 0), accum: 0)
print("Found easiest: \(waysFound.min()!)")