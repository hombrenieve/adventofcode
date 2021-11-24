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

var original = program

outerloop: for noun in 0...99 {
    for verb in 0...99 {
        print("Checking: noun=\(noun), verb=\(verb)")
        program = original
        program[1] = noun
        program[2] = verb
        runProgram(0)
        if program[0] == 19690720 {
            print("Found!: noun=\(noun), verb=\(verb), res=\(noun*100+verb)")
            break outerloop
        }
    }
}