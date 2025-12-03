import { readInput } from './utils';

console.log("Day 3 - Part 1");

// const lines = readInput('inputEx.txt');
const lines = readInput('inputOf.txt');
console.log(`Read ${lines.length} lines from input file`);

let sum = 0;

for (const line of lines) {
    let max = 0;
    for (let i = 0; i < line.length; i++) {
        let digit = line[i];
        for (let j = i + 1; j < line.length; j++) {
            let nextDigit = line[j];
            let nr = parseInt(digit + nextDigit);
            if (nr > max) {
                max = nr;
            }
        }
    }
    console.log(`Max two-digit number in line is ${max}`);
    sum += max;
}

console.log(`Sum of all max two-digit numbers: ${sum}`);