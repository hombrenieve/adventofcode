import { readInput } from './utils';

console.log("Day 12 - Part 1");

const lines = readInput('inputEx.txt');
// const lines = readInput('inputOf.txt');
console.log(`Read ${lines.length} lines from input file`);

class Shape {
    lines: string[];
    height: number;
    width: number;

    constructor(lines: string[]) {
        this.lines = lines;
        this.height = lines.length;
        this.width = lines[0].length;
    }
}

class Region {
    width: number = 0;
    height: number = 0;
    shapes: number[] = [];

    constructor(line: string) {
        let size = line.split(': ').at(0)?.split('x').map((n) => parseInt(n));
        let shapes = line.split(': ').at(1)?.split(' ').map((n) => parseInt(n));
        if (size && shapes) {
            this.width = size[0];
            this.height = size[1];
            this.shapes = shapes;
        }
    }
}

const shapes: Shape[] = [];
const regions: Region[] = [];

function read_shape(lines: string[], line: number) {
    const shape: string[] = [];
    while (line < lines.length && lines[line] !== "") {
        shape.push(lines[line]);
        line++;
    }
    return new Shape(shape);
}

let line = 0;
while (line < lines.length) {
    const l = lines[line];
    if(l === "") {
        line++;
        continue;
    }
    if (l.match(/^\d+:$/)) {
        shapes.push(read_shape(lines, line + 1));
        line += shapes.at(-1)!.height + 1;
    }
    else {
        regions.push(new Region(l));
        line++;
    }
}

console.log(`Read ${shapes.length} shapes and ${regions.length} regions`);