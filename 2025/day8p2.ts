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

// Add edges until all junction boxes are connected
let connectedBoxes: Set<JunctionBox> = new Set();

let lastEdge = null;

for (const edge of edges) {
    if (!connectedBoxes.has(edge.from) || !connectedBoxes.has(edge.to)) {
        connectedBoxes.add(edge.from);
        connectedBoxes.add(edge.to);
        lastEdge = edge;
    }
    if (connectedBoxes.size === junctionBoxes.length) {
        break;
    }
}

let result = (lastEdge?.from.x ?? 1) * (lastEdge?.to.x ?? 1);

console.log(`Result: ${result}`);
