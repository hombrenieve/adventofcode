import { readInput } from './utils';

console.log("Day 11 - Part 1");

const lines = readInput('inputOf.txt');
console.log(`Read ${lines.length} lines from input file`);

let adjacency = new Map<string, string[]>();

for(let line of lines) {
    const splitted = line.split(' ');
    const from = splitted[0].slice(0, splitted[0].length-1);
    for(let i = 1; i < splitted.length; i++) {
        const to = splitted[i];
        if(!adjacency.has(from)) {
            adjacency.set(from, []);
        }
        adjacency.get(from)?.push(to);
    }
}

// now that we have the adjacency list built we need to find the length of all paths from you to out
// we can do this by doing a bfs from you to out and counting the number of paths
let paths = 0;
let queue = ['you'];
while(queue.length > 0) {
    let current = queue.shift();
    if(current === 'out') {
        paths++;
        continue;
    }
    if(current === undefined) {
        continue;
    }
    if(adjacency.has(current)) {
        for(let neighbor of adjacency.get(current)!) {
            queue.push(neighbor);
        }
    }
}
console.log(`There are ${paths} paths`);