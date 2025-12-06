import { assert } from 'console';
import { readInput } from './utils';
import { get } from 'http';

console.log("Day 6 - Part 2");

const lines = readInput('inputOf.txt');
console.log(`Read ${lines.length} lines from input file`);

let operators = lines[lines.length-1].split(" ");
operators = operators.filter(op => op.trim() !== "");

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

function operateAll(operands: number[], operator: string): number {
    return operands.reduce((a, b) => operate(a, b, operator));
}

function composeNumber(index: number): string {
    let result = "";
    for (let j = 0; j < lines.length-1; j++) {
        result += lines[j].slice(index, index+1);
    }
    return result;
}

let results: number[] = [];
let operands: number[] = [];
let index = operators.length-1;
for (let i = lines[0].length; i >= 0; i--) {
    let numberStr = composeNumber(i-1);
    if (numberStr.trim() === "" || i === 0) {
        //console.log(`Operands at index ${index}: ${operands}`);
        results.push(operateAll(operands, operators[index]));
        index--;
        operands = [];
        continue;
    }
    let number = parseInt(numberStr);
    operands.push(number);
}

let grandTotal = results.reduce((a, b) => a + b, 0);
console.log(`Grand total: ${grandTotal}`);