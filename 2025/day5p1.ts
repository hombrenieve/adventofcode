import { readInput } from './utils';

console.log("Day 5 - Part 1");

const lines = readInput('inputOf.txt');
// const lines = readInput('inputOf.txt');
console.log(`Read ${lines.length} lines from input file`);

class Interval {
    start: number;
    end: number;

    constructor(start: number, end: number) {
        this.start = start;
        this.end = end;
    }

    contains(value: number): boolean {
        return value >= this.start && value <= this.end;
    }
}

let freshIngredients : Interval[] = [];
let line = lines[0];
let index = 0;
while (line !== "") {
    freshIngredients.push(new Interval(parseInt(line.split("-")[0]), parseInt(line.split("-")[1])));
    index++;
    line = lines[index];
}

let count = 0;
// Count all fresh ingredients in the next lines
for (let i = index + 1; i < lines.length; i++) {
    let ingredient = parseInt(lines[i]);
    for (let interval of freshIngredients) {
        if (interval.contains(ingredient)) {
            count++;
            break;
        }
    }
}

console.log(`Fresh ingredients count: ${count}`);