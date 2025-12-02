import { readInput } from './utils';

console.log("Day 2 - Part 2");

const lines = readInput('inputOf.txt');
const ranges = lines[0].split(',');
// const lines = readInput('inputOf.txt');
console.log(`Read ${lines.length} lines from input file`);

function isValidId(id: string): boolean {
    let current = id.slice(0,1);
    let i = 1;
    while (i < id.length) {
        if (current === id.slice(i, i + current.length )) {
            i += current.length;
        } else {
            current += id[current.length];
            i = current.length;
        }
    }
    if (current.length < id.length) {
        return false;
    }    
    return true;
}

function getRange(range: string): number[] {
    return range.split('-').map(Number);
}

let invalidIds: number[] = [];

for (const range of ranges) {
    const [start, end] = getRange(range);
    for (let id = start; id <= end; id++) {
        if (!isValidId(id.toString())) {
            invalidIds.push(id);
        }
    }
}

// add all invalid IDs
const sumInvalidIds = invalidIds.reduce((a, b) => a + b, 0);
console.log(`Sum of invalid IDs: ${sumInvalidIds}`);