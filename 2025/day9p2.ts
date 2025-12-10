import { readInput } from './utils';

console.log("Day 9 - Part 2");

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

let segments: Point[][] = [];
for (let i = 0; i < points.length; i++) {
    for (let j = i+1; j < points.length; j++) {
        const pointA = points[i];
        const pointB = points[j];
        if (pointA.x === pointB.x || pointA.y === pointB.y) {
            segments.push([pointA, pointB]);
        }
    }
}

function canFormBoundary(minCoord: number, maxCoord: number, isHorizontal: boolean, condition: (seg: Point[]) => boolean): boolean {
    let validSegs = segments.filter(condition);
    let ranges: [number, number][] = validSegs.map(seg => {
        let start = isHorizontal ? Math.min(seg[0].x, seg[1].x) : Math.min(seg[0].y, seg[1].y);
        let end = isHorizontal ? Math.max(seg[0].x, seg[1].x) : Math.max(seg[0].y, seg[1].y);
        return [start, end];
    });
    
    ranges.sort((a, b) => a[0] - b[0]);
    
    let covered = minCoord - 1;
    for (let [start, end] of ranges) {
        if (start <= covered + 1) {
            covered = Math.max(covered, end);
        }
    }
    
    return covered >= maxCoord;
}

let eligiblePoints = function (a: Point, b: Point) {
    let upper = a.y > b.y ? a : b;
    let lower = a.y < b.y ? a : b;
    let left = a.x < b.x ? a : b;
    let right = a.x > b.x ? a : b;

    let hasUpper = canFormBoundary(left.x, right.x, true, seg => seg[0].y >= upper.y && seg[1].y >= upper.y);
    let hasLower = canFormBoundary(left.x, right.x, true, seg => seg[0].y <= lower.y && seg[1].y <= lower.y);
    let hasLeft = canFormBoundary(lower.y, upper.y, false, seg => seg[0].x <= left.x && seg[1].x <= left.x);
    let hasRight = canFormBoundary(lower.y, upper.y, false, seg => seg[0].x >= right.x && seg[1].x >= right.x);

    return hasUpper && hasLower && hasLeft && hasRight;
};

// Calculate the biggest area
let maxArea = 0;

for (let i = 0; i < points.length; i++) {
    const pointA = points[i];

    for (let j = i+1; j < points.length; j++) {
        const pointB = points[j];
        if (!eligiblePoints(pointA, pointB)) {
            continue;
        }
        maxArea = Math.max(maxArea, pointA.area(pointB));
    }
}

console.log(`The biggest area is: ${maxArea}`);