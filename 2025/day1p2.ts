import { readInput } from './utils';

console.log("Day 1 - Part 2");

const lines = readInput('inputOf.txt');
console.log(`Read ${lines.length} lines from input.txt`);

const max_dial = 100;
let dial = 50;

let ceroes = 0;


for (const line of lines) {
    const direction = line[0];
    const amount = parseInt(line.slice(1), 10);

    if (direction === 'L') {
        const step = amount % max_dial;
        let newDial = (dial - step + max_dial) % max_dial;
        // Count how many times we hit position 0 while moving left.
        // For left moves, k in 1..amount hits 0 when k % max_dial === dial.
        // If start (dial) is 0 the count is floor(amount / max_dial).
        let crossings = 0;
        if (dial === 0) {
            crossings = Math.floor(amount / max_dial);
        } else if (amount >= dial) {
            crossings = 1 + Math.floor((amount - dial) / max_dial);
        }
        if (crossings > 0) {
            console.log(`Dial goes through zero moving left from ${dial} to ${newDial} -> ${crossings} times`);
            ceroes += crossings;
        }
        dial = newDial;
    } else if (direction === 'R') {
        const step = amount % max_dial;
        let newDial = (dial + step) % max_dial;
        // Count how many times we hit position 0 while moving right.
        // For right moves, number of zero hits is floor((start + amount) / max_dial).
        const crossings = Math.floor((dial + amount) / max_dial);
        if (crossings > 0) {
            console.log(`Dial goes through zero moving right from ${dial} to ${newDial} -> ${crossings} times`);
            ceroes += crossings;
        }
        dial = newDial;
    }

    console.log(`Dial is now at position: ${dial}`);
}

console.log(`The dial hit position 0 a total of ${ceroes} times.`);