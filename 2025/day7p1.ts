import { readInput } from './utils';

console.log("Day 7 - Part 1");

const lines = readInput('inputOf.txt');
console.log(`Read ${lines.length} lines from input file`);

class PointSet {
    private items: Point[];

    constructor(items?: Point[]) {
        this.items = items ?? [];
    }

    add(item: Point): void {
        for (const existing of this.items) {
            if (existing.equals(item)) {
                return;
            }
        }
        this.items.push(item);
    }

    has(item: Point): boolean {
        for (const existing of this.items) {
            if (existing.equals(item)) {
                return true;
            }
        }
        return false;
    }

    values(): Point[] {
        return [...this.items];
    }

    size(): number {
        return this.items.length;
    }
}

class Point {
    x: number;
    y: number;
    
    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }

    equals(other: Point): boolean {
        return this.x === other.x && this.y === other.y;
    }
}

function buildSplitters(lines: string[]): Point[] {
    const splitters: Point[] = [];
    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        for (let j = 0; j < line.length; j++) {
            if (line.slice(j, j+1) === "^") {
                splitters.push(new Point(j, i));
            }
        }
    }
    return splitters;
}

class Beam {
    currentPositions: PointSet;
    splitters: PointSet;
    limits: Point;
    splittedBeams: number = 0;
    currentDepth : number = 0;

    constructor(starting: Point, limits: Point, splitters: Point[]) {
        this.currentPositions = new PointSet([starting]);
        this.splitters = new PointSet(splitters);
        this.limits = limits;
    }

    move(): boolean {
        // Move all current positions down by one if an splitter is found the beam splits
        // respecting the limits and counting one beam per splitter found
        const newPositions = new PointSet();
        for (const pos of this.currentPositions.values()) {
            const below = new Point(pos.x, pos.y + 1);
            if (below.y >= this.limits.y) {
                // We are finished
                return false;
            }
            if (this.splitters.has(below)) {
                let hasSplit = false;
                // Split the beam
                if (pos.x > 0) {
                    newPositions.add(new Point(pos.x - 1, pos.y + 1));
                    hasSplit = true;
                }
                if (pos.x < this.limits.x - 1) {
                    newPositions.add(new Point(pos.x + 1, pos.y + 1));
                    hasSplit = true;
                }
                if (hasSplit) {
                    this.splittedBeams++;
                }
            } else {
                // Just move down
                newPositions.add(below);
            }
        }
        this.currentPositions = newPositions;
        this.currentDepth++;
        return true;
    }

    getNumberOfBeams(): number {
        return this.currentPositions.size();
    }

    getNumberOfSplittedBeams(): number {
        return this.splittedBeams;
    }
}

let beam = new Beam(new Point(Math.floor(lines[0].length/2), 0), new Point(lines[0].length, lines.length), buildSplitters(lines));

while (beam.move()) {
    console.log(`Depth ${beam.currentDepth} Concurrent beams: ${beam.getNumberOfBeams()}`);
}

console.log(`Total splitted beams: ${beam.getNumberOfSplittedBeams()}`);