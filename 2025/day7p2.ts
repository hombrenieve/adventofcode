import { readInput } from './utils';

console.log("Day 7 - Part 2");

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

class BeamPath {
    positions: Point[];

    constructor(positions: Point[] = []) {
        this.positions = positions;
    }

    // Create a string representation of the path for comparison
    pathKey(): string {
        return this.positions.map(p => `${p.x},${p.y}`).join('|');
    }

    // Add a position to the path
    addPosition(p: Point): BeamPath {
        return new BeamPath([...this.positions, p]);
    }

    getCurrentPosition(): Point {
        return this.positions[this.positions.length - 1];
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
    startingPoint: Point;
    splitters: PointSet;
    limits: Point;

    constructor(startingPoint: Point, limits: Point, splitters: Point[]) {
        this.startingPoint = startingPoint;
        this.splitters = new PointSet(splitters);
        this.limits = limits;
    }

    findTimelines(): number {
        // Map to track how many ways we can reach each position
        // Key: "x,y", Value: number of ways to reach that position
        let ways: Map<string, number> = new Map();
        ways.set(`${this.startingPoint.x},${this.startingPoint.y}`, 1);

        // Process each row from top to bottom
        for (let y = this.startingPoint.y; y < this.limits.y - 1; y++) {
            const nextWays: Map<string, number> = new Map();

            for (const [posKey, count] of ways.entries()) {
                const [x] = posKey.split(',').map(Number);
                const pos = new Point(x, y);

                // Check if there's a splitter at this position
                if (this.splitters.has(pos)) {
                    // Left branch
                    if (x > 0) {
                        const leftKey = `${x - 1},${y + 1}`;
                        nextWays.set(leftKey, (nextWays.get(leftKey) || 0) + count);
                    }
                    // Right branch
                    if (x < this.limits.x - 1) {
                        const rightKey = `${x + 1},${y + 1}`;
                        nextWays.set(rightKey, (nextWays.get(rightKey) || 0) + count);
                    }
                } else {
                    // No splitter, continue straight down
                    const downKey = `${x},${y + 1}`;
                    nextWays.set(downKey, (nextWays.get(downKey) || 0) + count);
                }
            }

            ways = nextWays;
        }

        // Count total paths reaching the bottom
        let totalPaths = 0;
        for (const count of ways.values()) {
            totalPaths += count;
        }

        return totalPaths;
    }
}

let beam = new Beam(new Point(Math.floor(lines[0].length/2), 0), new Point(lines[0].length, lines.length), buildSplitters(lines));

const result = beam.findTimelines();

console.log(`Total timelinses: ${result}`);