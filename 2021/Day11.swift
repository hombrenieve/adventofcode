import Foundation

typealias Point = (Int, Int)

struct DumboOctopuses {
    var array: [[Int]]
    var total: Int {
        return array.count*array[0].count
    }
    

    init(_ read: [String]) {
        array = read.map({$0.map({$0.wholeNumberValue!})})
    }


    private mutating func increaseEnergy(_ point: Point) {
        let (x, y) = point
        //check out of bounds
        guard x >= 0, x < array[0].count, y >= 0, y < array.count else {return}
        if array[y][x] < 10 { //has not flashed yet
            array[y][x] += 1
            if(array[y][x] == 10) {//it flashes
                for i in -1...1 {
                    for j in -1...1 {
                        if j == 0 && i == 0 { continue }
                        increaseEnergy((x+i, y+j))
                    }
                }
            }
        }
    }


    mutating func nextStep() -> Int {
        for j in 0...array.count-1 {
            for i in 0...array[j].count-1 {
                increaseEnergy((i, j))
            }
        }
        var flashed = 0
        for j in 0...array.count-1 {
            for i in 0...array[j].count-1 {
                if array[j][i] == 10 {
                    flashed += 1
                    array[j][i] = 0
                }
            }
        }
        return flashed
    }

    mutating func calculateFlashedIn(steps: Int) -> Int {
        var totalFlashed = 0
        for _ in 1...steps {
            totalFlashed += self.nextStep()
        }
        return totalFlashed
    }

    mutating func findSynchStep() -> Int {
        var flashed = 0
        var step = 0
        repeat {
            flashed = self.nextStep()
            step += 1
        } while flashed != total
        return step
    }
}

var input: [String] = []

while let line = readLine() {
    input.append(line)
}

var octopuses = DumboOctopuses(input)

print("All flashed in step: \(octopuses.findSynchStep())")
