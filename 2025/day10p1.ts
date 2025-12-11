import { readInput } from './utils';

console.log("Day 10 - Part 1");

const lines = readInput('inputOf.txt');
console.log(`Read ${lines.length} lines from input file`);

class GameTree {
    initialState: string;
    desiredState: string;
    buttons: number[][];
    graph: Map<string, Map<string, number>>; // adjacency list with edge weights (button press count)
    
    constructor(desiredState: string, buttons: string[]) {
        this.desiredState = desiredState;
        this.initialState = desiredState.replace(/#/g, '.');
        // buttons come in a string with the format (3,4,5) (5,6,7) etc
        // we need to convert this to an array of arrays of numbers
        this.buttons = buttons.map(button => button.replace(/[()]/g, '').split(',').map(Number));
        this.graph = new Map();
        
        console.log(`Initial state: ${this.initialState}`);
        console.log(`Desired state: ${this.desiredState}`);
        console.log(`Buttons: ${JSON.stringify(this.buttons)}`);
    }

    transition(state: string, button: number[]): string {
        let newState = state.split('');
        for (let i = 0; i < button.length; i++) {
            const pos = button[i];
            if (pos >= 0 && pos < newState.length) {
                newState[pos] = newState[pos] === '.' ? '#' : '.';
            }
        }
        return newState.join('');
    }

    buildGraph(): void {
        const visited = new Set<string>();
        const queue = [this.initialState];
        visited.add(this.initialState);

        while (queue.length > 0) {
            const currentState = queue.shift()!;
            
            if (!this.graph.has(currentState)) {
                this.graph.set(currentState, new Map());
            }

            // Try each button from current state
            for (const button of this.buttons) {
                const nextState = this.transition(currentState, button);
                
                // Add edge from current to next state (weight = 1 button press)
                this.graph.get(currentState)!.set(nextState, 1);
                
                // If we haven't visited this state, add it to queue
                if (!visited.has(nextState)) {
                    visited.add(nextState);
                    queue.push(nextState);
                }
            }
        }
    }

    findShortestPath(): { path: string[], buttonPresses: number } | null {
        if (!this.graph.has(this.initialState)) {
            return null;
        }

        const distances = new Map<string, number>();
        const previous = new Map<string, string>();
        const unvisited = new Set<string>();

        // Initialize distances
        for (const state of this.graph.keys()) {
            distances.set(state, Infinity);
            unvisited.add(state);
        }
        distances.set(this.initialState, 0);

        while (unvisited.size > 0) {
            // Find unvisited node with minimum distance
            let current: string | null = null;
            let minDistance = Infinity;
            for (const state of unvisited) {
                const dist = distances.get(state)!;
                if (dist < minDistance) {
                    minDistance = dist;
                    current = state;
                }
            }

            if (current === null || minDistance === Infinity) {
                break; // No path exists
            }

            unvisited.delete(current);

            // If we reached the desired state, reconstruct path
            if (current === this.desiredState) {
                const path: string[] = [];
                let node: string | undefined = current;
                while (node !== undefined) {
                    path.unshift(node);
                    node = previous.get(node);
                }
                return { path, buttonPresses: distances.get(current)! };
            }

            // Update distances to neighbors
            const neighbors = this.graph.get(current);
            if (neighbors) {
                for (const [neighbor, weight] of neighbors) {
                    if (unvisited.has(neighbor)) {
                        const newDistance = distances.get(current)! + weight;
                        if (newDistance < distances.get(neighbor)!) {
                            distances.set(neighbor, newDistance);
                            previous.set(neighbor, current);
                        }
                    }
                }
            }
        }

        return null; // No path found
    }

    solve(): number {
        console.log(`Building graph from initial state: ${this.initialState}`);
        console.log(`Target state: ${this.desiredState}`);
        
        this.buildGraph();
        console.log(`Graph built with ${this.graph.size} states`);
        
        const result = this.findShortestPath();
        if (result) {
            console.log(`Shortest path found with ${result.buttonPresses} button presses`);
            console.log(`Path: ${result.path.join(' -> ')}`);
            return result.buttonPresses;
        } else {
            console.log('No path found from initial to desired state');
            return -1;
        }
    }
}

let minimumPaths = 0;

for (let line of lines) {
    console.log(`Processing line: ${line}`);
    
    // Split by spaces but handle the parsing more carefully
    const parts = line.split(' ');
    
    // Extract desired state (remove brackets)
    const desiredState = parts[0].replace(/[\[\]]/g, '');
    
    // Extract buttons - everything that starts with ( and ends with ), but stop at {
    const buttons: string[] = [];
    for (let i = 1; i < parts.length; i++) {
        const part = parts[i];
        if (part.startsWith('{')) {
            break; // Stop when we hit the curly braces section
        }
        if (part.startsWith('(') && part.endsWith(')')) {
            buttons.push(part.slice(1, -1)); // Remove parentheses
        }
    }
    
    console.log(`Desired state: ${desiredState}`);
    console.log(`Buttons: ${JSON.stringify(buttons)}`);
    
    let gameTree = new GameTree(desiredState, buttons);
    const minButtonPresses = gameTree.solve();
    if (minButtonPresses > 0) {
        minimumPaths += minButtonPresses;
    }
    console.log(`Minimum button presses: ${minButtonPresses}`);
    console.log('---');
}

console.log(`Minimum button presses: ${minimumPaths}`);