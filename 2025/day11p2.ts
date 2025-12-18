import { readInput } from './utils';

console.log("Day 11 - Part 2 (DP)");

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



// Count paths using DP with memoization and cycle detection
function countPaths(
    adjacency: Map<string, string[]>, 
    start: string, 
    end: string
): number {
    console.log(`  Counting paths from ${start} to ${end}...`);
    
    const memo = new Map<string, number>();
    const computing = new Set<string>();
    
    function dfs(node: string): number {
        if (node === end) return 1;
        
        if (memo.has(node)) return memo.get(node)!;
        
        // Cycle detection - if we're already computing this node, it's a cycle
        if (computing.has(node)) return 0;
        
        computing.add(node);
        
        let totalPaths = 0;
        if (adjacency.has(node)) {
            for (const neighbor of adjacency.get(node)!) {
                totalPaths += dfs(neighbor);
            }
        }
        
        computing.delete(node);
        memo.set(node, totalPaths);
        return totalPaths;
    }
    
    const result = dfs(start);
    console.log(`  Found ${result} paths from ${start} to ${end}`);
    return result;
}



// Main function
function main() {
    console.log(`\nGraph has ${adjacency.size} nodes`);
    console.log(`svr connects to: ${adjacency.get('svr')?.join(', ') || 'none'}`);
    console.log(`dac connects to: ${adjacency.get('dac')?.join(', ') || 'none'}`);
    console.log(`fft connects to: ${adjacency.get('fft')?.join(', ') || 'none'}`);
    
    console.log(`\n=== DP Path Counting ===`);
    
    // Count paths using DP with memoization
    const pathsFftToDac = countPaths(adjacency, 'fft', 'dac');
    const pathsSvrToFft = countPaths(adjacency, 'svr', 'fft');
    const pathsDacToOut = countPaths(adjacency, 'dac', 'out');
    
    // Calculate total for the viable route
    if (pathsFftToDac > 0) {
        const totalPaths = pathsSvrToFft * pathsFftToDac * pathsDacToOut;
        console.log(`\n=== RESULT ===`);
        console.log(`Route: svr -> fft -> dac -> out`);
        console.log(`Total paths: ${pathsSvrToFft} × ${pathsFftToDac} × ${pathsDacToOut} = ${totalPaths}`);
    } else {
        console.log(`\nNo paths from fft to dac, so no complete paths through both nodes.`);
    }
}

main();