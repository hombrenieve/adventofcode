import Foundation

struct BoardSums {
    var rows = [Int](repeating:0, count: 5)
    var columns = [Int](repeating:0, count: 5)
    var isBingo: Bool {
        let inRow = rows.firstIndex(of: 0)
        let inColumn = columns.firstIndex(of: 0)
        return inRow != nil || inColumn != nil
    }
    var remaining: Int {
        return rows.reduce(0, +)
    }
}

class BingoBoard {
    var data: [[Int]]
    init(_ input: [[Int]]) {
        data = input
        
    }
    var sums: BoardSums {
        var sums = BoardSums()
        for i in 0...4 {
            sums.rows[i] = data[i].filter({$0 != -1}).reduce(0, +)
            for j in 0...4 {
                if data[i][j] != -1 {
                    sums.columns[j] += data[i][j]
                }
            }
        }
        return sums
    }
    func remove(number: Int) -> Bool {
        for i in 0...4 {
            if let ind = data[i].firstIndex(of: number) {
                data[i][ind] = -1
                return true
            }
        }
        return false
    }
}



//Game
let bingoGame = readLine()!.components(separatedBy: ",").map({Int($0)!})
var bingoBoards : [BingoBoard] = []

while let _ = readLine() {
    //First empty line
    var inputString : [String] = []
    for _ in 1...5 {
        inputString.append(readLine()!)
    }
    bingoBoards.append(BingoBoard(inputString.map({$0.components(separatedBy: " ").filter({$0 != ""}).map({Int($0)!})})))
}

for ball in bingoGame {
    bingoBoards.forEach({$0.remove(number: ball)})
    if let winner = bingoBoards.firstIndex(where: {$0.sums.isBingo}) {
        let remaining = bingoBoards[winner].sums.remaining
        print("With ball: \(ball) The board \(winner) has bingo, remaining: \(remaining)")
        print("Result is: \(ball*remaining)")
        break
    }
}
