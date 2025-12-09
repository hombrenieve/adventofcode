import { readInput } from './utils';

console.log("Day 9 - Part 1");

const lines = readInput('inputOf.txt');
console.log(`Read ${lines.length} lines from input file`);

class Point {
    x: number;
    y: number;

    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }

    area(other: Point): number {
        return (Math.abs(this.x - other.x) + 1) * (Math.abs(this.y - other.y) + 1);
    }
}

let points: Point[] = [];

for (const line of lines) {
    const [x, y] = line.split(',').map(Number);
    points.push(new Point(x, y));
}

// Calculate the biggest area
let maxArea = 0;

for (let i = 0; i < points.length; i++) {
    const pointA = points[i];

    for (let j = i+1; j < points.length; j++) {
        const pointB = points[j];
        maxArea = Math.max(maxArea, pointA.area(pointB));
    }
}

console.log(`The biggest area is: ${maxArea}`);