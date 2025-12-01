import { readInput } from './utils';

console.log("Day 1 - Part 1");

const lines = readInput();
console.log(`Read ${lines.length} lines from input.txt`);

const max_dial = 100;
let dial = 50;

let ceroes = 0;

for (const line of lines) {
    const direction = line[0];
    const amount = parseInt(line.slice(1), 10);

    if (direction === 'L') {
        // the minimum number is 0 and the maximum number is max_dial - 1
        // I want to wrap around if I go below 0 but amount might be
        // larger several times than max_dial
        // reduce the movement to within one full rotation to avoid
        // negative remainders when using the % operator
        const step = amount % max_dial;
        dial = (dial - step + max_dial) % max_dial;
    } else if (direction === 'R') {
        const step = amount % max_dial;
        dial = (dial + step) % max_dial;
    }

    console.log(`Dial is now at position: ${dial}`);

    if (dial === 0) {
        ceroes += 1;
    }
}

console.log(`The dial hit position 0 a total of ${ceroes} times.`);