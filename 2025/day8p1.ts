import { readInput } from './utils';

console.log("Day 8 - Part 1");

const lines = readInput('inputEx.txt');
console.log(`Read ${lines.length} lines from input file`);

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

    contains(box: JunctionBox): boolean {
        return (this.from === box || this.to === box);
    }

    linksTogether(box1: JunctionBox, box2: JunctionBox): boolean {
        return (this.from === box1 && this.to === box2) ||
               (this.from === box2 && this.to === box1);
    }
    getLength(): number {
        return this.length;
    }
}

class Circuit {
    edges: Edge[] = [];

    addEdge(edge: Edge): void {
        this.edges.push(edge);
    }

    belongsToCircuit(edge: Edge): number {
        // I want to count if one of the edge's junction boxes is already in this circuit
        // or both of them, but I only need to count them once, it means tha the result is either 0, 1 or 2
        let isFrom = false;
        for (const existingEdge of this.edges) {
            if (existingEdge.contains(edge.from)) {
                isFrom = true;
                break;
            }
        }
        let isTo = false;
        for (const existingEdge of this.edges) {
            if (existingEdge.contains(edge.to)) {
                isTo = true;
                break;
            }
        }
        if (isFrom && isTo) {
            return 2;
        } else if (isFrom || isTo) {
            return 1;
        } else {
            return 0;
        }
    }

    getTotalJunctionBoxes(): number {
        const boxes = new Set<JunctionBox>();
        for (const edge of this.edges) {
            boxes.add(edge.from);
            boxes.add(edge.to);
        }
        return boxes.size;
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

console.log(`Generated ${edges.length} edges between junction boxes.`);

const CONNECTIONS = 10;
let connectionsEstablished = 0;
let currentEdgeIndex = 0;

let circuits: Circuit[] = [];
while (connectionsEstablished < CONNECTIONS) {
    const edge = edges[currentEdgeIndex];
    currentEdgeIndex++;
    let addedToExistingCircuit = false;
    for (const circuit of circuits) {
        switch (circuit.belongsToCircuit(edge)) {
            case 2:
                // Both junction boxes are already in this circuit, skip this edge
                addedToExistingCircuit = true;
                circuit.addEdge(edge);
                break;
            case 1:
                // One junction box is in this circuit, add the edge
                circuit.addEdge(edge);
                connectionsEstablished++;
                addedToExistingCircuit = true;
                break;
            case 0:
                // Neither junction box is in this circuit, do nothing
                break;
        }
    }
    // If no existing circuit contains this edge, create a new one
    if (addedToExistingCircuit) {
        continue;
    }
    const newCircuit = new Circuit();
    newCircuit.addEdge(edge);
    circuits.push(newCircuit);
    connectionsEstablished++;
}

// Sort circuits by number of junction boxes
circuits.sort((a, b) => b.getTotalJunctionBoxes() - a.getTotalJunctionBoxes());

console.log(`Identified ${circuits.length} circuits.`);
// Show size of each circuit
for (let i = 0; i < circuits.length; i++) {
    console.log(`Circuit ${i + 1}: ${circuits[i].getTotalJunctionBoxes()} junction boxes.`);
}

// Multiply the size of the 3 first circuits
let result = 1;
for (let i = 0; i < 3; i++) {
    result *= circuits[i].getTotalJunctionBoxes();
}

console.log(`Result: ${result}`);
