import { readInput } from './utils';

console.log("Day 4 - Part 1");

const lines = readInput('inputOf.txt');
console.log(`Read ${lines.length} lines from input file`);

function get_adjacent(x: number, y: number): number {
    let count = 0;
    for (let dx = -1; dx <= 1; dx++) {
        for (let dy = -1; dy <= 1; dy++) {
            if (dx === 0 && dy === 0) continue;
            if (x + dx < 0 || y + dy < 0) continue;
            if (x + dx >= lines[0].length || y + dy >= lines.length) continue;
            if (lines[y + dy]?.[x + dx] === '@') {
                count++;
            }
        }
    }
    return count;
}

let movable = 0;
for (let y = 0; y < lines.length; y++) {
    for (let x = 0; x < lines[y].length; x++) {
        if (lines[y][x] !== '@') continue;
        if(get_adjacent(x, y) < 4) {
            movable++;
        }
    }
}

console.log(`Movable rolls: ${movable}`);