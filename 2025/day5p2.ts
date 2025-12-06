import { readInput } from './utils';

console.log("Day 5 - Part 2");

const lines = readInput('inputOf.txt');
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

    intersection(other: Interval): Interval | null {
        const newStart = Math.max(this.start, other.start);
        const newEnd = Math.min(this.end, other.end);
        if (newStart <= newEnd) {
            return new Interval(newStart, newEnd);
        }
        return null;
    }

    substract(other: Interval): Interval[] {
        const result: Interval[] = [];
        if (other.start > this.start) {
            result.push(new Interval(this.start, Math.min(this.end, other.start - 1)));
        }
        if (other.end < this.end) {
            result.push(new Interval(Math.max(this.start, other.end + 1), this.end));
        }
        return result;
    }

    disjoint(other: Interval): [Interval, Interval, Interval] | null {
        const intersection = this.intersection(other);
        if (intersection) {
            const remainingThis = this.substract(intersection);
            const remainingOther = other.substract(intersection);
            return [remainingThis, intersection, remainingOther].flat() as [Interval, Interval, Interval];
        }
        return null;
    }
    
    length(): number {
        return this.end - this.start + 1;
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

let intersectionsFound = true;
while (intersectionsFound) {
    intersectionsFound = false;
    for (let i = 0; i < freshIngredients.length; i++) {
        for (let j = i + 1; j < freshIngredients.length; j++) {
            //console.log(`Checking intervals ${freshIngredients[i].start}-${freshIngredients[i].end} and ${freshIngredients[j].start}-${freshIngredients[j].end}`);
            const disjoint = freshIngredients[i].disjoint(freshIngredients[j]);
            if (disjoint) {
                intersectionsFound = true;
                freshIngredients = freshIngredients.filter((_, idx) => idx !== i && idx !== j);
                freshIngredients.push(...disjoint);
                // for(const interval of freshIngredients) {
                //     console.log(` - ${interval.start}-${interval.end}`);
                // }
                break;
            }
        }
        if (intersectionsFound) {
            break;
        }
    }
}

//console.log(`Intervals of fresh ingredients:`);
let intervalsSize = 0;
for (const interval of freshIngredients) {
    //console.log(` - ${interval.start}-${interval.end} (size: ${interval.length()})`);
    intervalsSize += interval.length();
}

console.log(`Total size of fresh ingredients intervals: ${intervalsSize}`);
