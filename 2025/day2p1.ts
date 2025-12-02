import { readInput } from './utils';

console.log("Day 2 - Part 1");

const lines = readInput('inputOf.txt');
const ranges = lines[0].split(',');
// const lines = readInput('inputOf.txt');
console.log(`Read ${lines.length} lines from input file`);

function isValidId(id: string, pre: string): boolean {
    if(id.length === 0) return true;
    if(id === pre) return false;
    const current = pre + id[0];
    return isValidId(id.slice(1), current);
}

function getRange(range: string): number[] {
    return range.split('-').map(Number);
}

let invalidIds: number[] = [];

for (const range of ranges) {
    const [start, end] = getRange(range);
    for (let id = start; id <= end; id++) {
        if (!isValidId(id.toString(), '')) {
            invalidIds.push(id);
        }
    }
}

// add all invalid IDs
const sumInvalidIds = invalidIds.reduce((a, b) => a + b, 0);
console.log(`Sum of invalid IDs: ${sumInvalidIds}`);