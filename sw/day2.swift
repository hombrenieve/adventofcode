import Foundation

var program: [Int] = []

let opcodes: [Int: (Int,Int) -> Int] = [
    1: { a, b in return a+b },
    2: { a, b in return a*b }
]

func applyOperation(_ lhs: Int, _ rhs: Int, _ store: Int, _ operation: (Int, Int) -> Int) {
    program[store] = operation(program[lhs], program[rhs])
}

func runProgram(_ counter: Int) {
    if program[counter] != 99 {
        applyOperation(program[counter+1], program[counter+2], program[counter+3], opcodes[program[counter]]!)
        runProgram(counter+4)
    }
}


while let input = readLine() {
    let strs = input.components(separatedBy: [","])
    program.append(contentsOf: strs.compactMap{Int($0)})
}
print("Program before: \(program)")
runProgram(0)
print("Program after: \(program)")