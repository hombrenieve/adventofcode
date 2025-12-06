import { assert } from 'console';
import { readInput } from './utils';

console.log("Day 6 - Part 1");

const lines = readInput('inputOf.txt');
console.log(`Read ${lines.length} lines from input file`);

let operations = lines[0].split(" ").map(op => parseInt(op));
let operators = lines[lines.length-1].split(" ");
//remove blank entries
operations = operations.filter(op => !isNaN(op));
operators = operators.filter(op => op.trim() !== "");
assert(operations.length === operators.length, `Number of operations (${operations.length}) does not match number of operators (${operators.length})`);

function operate(a: number, b: number, operator: string): number {
    switch(operator) {
        case "+":
            return a + b;
        case "*":
            return a * b;
        default:
            throw new Error(`Unknown operator: ${operator}`);
    }
}

for (let i = 1; i < lines.length-1; i++) {
    let operands = lines[i].split(" ").map(op => parseInt(op));
    operands = operands.filter(op => !isNaN(op));
    assert(operands.length === operations.length, `Number of operands (${operands.length}) does not match number of operations (${operations.length}) at line ${i}`);
    for (let j = 0; j < operations.length; j++) {
        operations[j] = operate(operations[j], operands[j], operators[j]);
    }
}

let grandTotal = operations.reduce((a, b) => a + b, 0);
console.log(`Grand total: ${grandTotal}`);