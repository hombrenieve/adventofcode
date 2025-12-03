import { readInput } from './utils';

console.log("Day 3 - Part 2");

//const lines = readInput('inputEx.txt');
const lines = readInput('inputOf.txt');
console.log(`Read ${lines.length} lines from input file`);

let sum = 0;

for (const line of lines) {
    // Build the maximum subsequence of length 12 using a greedy stack approach
    let digits = '';
    for (let i = 0; i < line.length; i++) {
        const digit = line[i];
        // While we can remove the last digit and replace it with a bigger one
        // and still be able to reach a total length of 12, pop it.
        while (
            digits.length > 0 &&
            parseInt(digit) > parseInt(digits[digits.length - 1]) &&
            (line.length - i) > (12 - digits.length)
        ) {
            digits = digits.slice(0, -1);
        }

        if (digits.length < 12) {
            digits += digit;
        }
    }

    const bestTwelveDigits = parseInt(digits.slice(0, 12));
    console.log(`Best twelve-digit number from line: ${bestTwelveDigits}`);
    sum += bestTwelveDigits;
}

console.log(`Sum of all max twelve-digit numbers: ${sum}`);