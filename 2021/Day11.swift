import Foundation

typealias Point = (Int, Int)

struct DumboOctopuses {
    let array: [[Int]]
    var total: Int {
        return array.count*array[0].count
    }
    var flashed: Int {
        return array.map({$0.filter({$0 == 0}).count}).reduce(0,+)
    }
    

    init(_ read: [String]) {
        array = read.map({$0.map({$0.wholeNumberValue!})})
    }

    init(other: [[Int]]) {
        array = other
    }


    private func increaseEnergy(_ point: Point) -> DumboOctopuses {
        let (x, y) = point
        //check out of bounds
        guard x >= 0, x < array[0].count, y >= 0, y < array.count else {return self}
        if array[y][x] < 10 { //has not flashed yet
            var copy = array
            copy[y][x] += 1
            var dumbo = DumboOctopuses(other: copy)
            if(dumbo.array[y][x] == 10) {//it flashes
                for i in -1...1 {
                    for j in -1...1 {
                        if j == 0 && i == 0 { continue }
                        dumbo = dumbo.increaseEnergy((x+i, y+j))
                    }
                }
            }
            return dumbo
        }
        return self
    }

    func reset() -> DumboOctopuses {
        return DumboOctopuses(other: array.map({$0.map({if $0 == 10 { return 0 } else {return $0}})}))
    }


    func nextStep() -> DumboOctopuses {
        var dumbo = DumboOctopuses(other: self.array)
        for j in 0...array.count-1 {
            for i in 0...array[j].count-1 {
                dumbo = dumbo.increaseEnergy((i, j))
            }
        }
        return dumbo.reset()
    }

    func calculateFlashedIn(steps: Int) -> DumboOctopuses {
        var dumbo = self
        for _ in 1...steps {
            dumbo = dumbo.nextStep()
        }
        return dumbo
    }

    func findSynchStep() -> (Int, DumboOctopuses) {
        var dumbo = self
        var step = 0
        repeat {
            dumbo = dumbo.nextStep()
            step += 1
        } while dumbo.flashed != self.total
        return (step, dumbo)
    }
}

var input: [String] = []

while let line = readLine() {
    input.append(line)
}

let octopuses = DumboOctopuses(input)

print("All flashed in step: \(octopuses.findSynchStep().0)")
