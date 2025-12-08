import { readInput } from './utils';

const lines = readInput('inputOf.txt');

class JunctionBox {
    x: number;
    y: number;
    z: number;

    constructor(x: number, y: number, z: number) {
        this.x = x;
        this.y = y;
        this.z = z;
    }
}

class Edge {
    from: JunctionBox;
    to: JunctionBox
    length: number;

    constructor(from: JunctionBox, to: JunctionBox) {
        this.from = from;
        this.to = to;
        this.length = Math.sqrt(
            Math.pow(to.x - from.x, 2) +
            Math.pow(to.y - from.y, 2) +
            Math.pow(to.z - from.z, 2)
        );
    }
    getLength(): number {
        return this.length;
    }
}


let junctionBoxes: JunctionBox[] = [];
for (const line of lines) {
    const [xStr, yStr, zStr] = line.split(',');
    const box = new JunctionBox(parseInt(xStr), parseInt(yStr), parseInt(zStr));
    junctionBoxes.push(box);
}

// Generate all edges between junction boxes
let edges: Edge[] = [];
for (let i = 0; i < junctionBoxes.length; i++) {
    for (let j = i + 1; j < junctionBoxes.length; j++) {
        const edge = new Edge(junctionBoxes[i], junctionBoxes[j]);
        edges.push(edge);
    }
}

// Sort edges by length
edges.sort((a, b) => a.getLength() - b.getLength());

// edges generated

const CONNECTIONS = 1000;
const selectedEdges: Edge[] = [];

// Select the first N edges by length (including those that connect already-connected nodes)
selectedEdges.push(...edges.slice(0, Math.min(CONNECTIONS, edges.length)));

// Build groups from the selected edges
const parentAlt: number[] = Array.from({ length: junctionBoxes.length }, (_, i) => i);
function findAlt(i: number): number { if (parentAlt[i] !== i) parentAlt[i] = findAlt(parentAlt[i]); return parentAlt[i]; }
function unionAlt(a: number, b: number) {
    const ra = findAlt(a); const rb = findAlt(b);
    if (ra === rb) return;
    parentAlt[rb] = ra;
}
for (const e of selectedEdges) {
    const a = junctionBoxes.indexOf(e.from);
    const b = junctionBoxes.indexOf(e.to);
    if (a !== -1 && b !== -1) unionAlt(a, b);
}
const groupsAlt = new Map<number, Set<string>>();
for (let i = 0; i < junctionBoxes.length; i++) {
    const r = findAlt(i);
    if (!groupsAlt.has(r)) groupsAlt.set(r, new Set());
    groupsAlt.get(r)!.add(`${junctionBoxes[i].x},${junctionBoxes[i].y},${junctionBoxes[i].z}`);
}
// (groupsAlt contains the connected components built from selected edges)

// Compute sizes from groups built from selected edges
const sizes = Array.from(groupsAlt.values()).map(s => s.size).sort((a, b) => b - a);
console.log(`Identified ${sizes.length} circuits.`);
for (let i = 0; i < sizes.length; i++) {
    console.log(`Circuit ${i + 1}: ${sizes[i]} junction boxes.`);
}

// Multiply the size of the 3 largest circuits (or fewer if not present)
let result = 1;
for (let i = 0; i < Math.min(3, sizes.length); i++) {
    result *= sizes[i];
}

console.log(`Result: ${result}`);
