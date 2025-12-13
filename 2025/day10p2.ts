import { readInput } from './utils';

console.log("Day 10 - Part 2");

const lines = readInput('inputOf.txt');
console.log(`Read ${lines.length} lines from input file`);

/**
 * Main solver function - tries to find minimum button presses to reach exact targets
 */
function solve(buttons: number[][], target: number[]): number | null {
    // Start with all targets needing to be satisfied
    const remainingTargets = [...target];
    
    // All buttons are initially available (using bitmask for efficiency)
    const allButtonsAvailable = (1 << buttons.length) - 1;
    
    const result = findMinimumPresses(remainingTargets, allButtonsAvailable, buttons);
    
    return result === Infinity ? null : result;
}

/**
 * Core recursive function that finds minimum button presses
 * 
 * @param targets - Array showing how much each target still needs
 * @param availableButtons - Bitmask showing which buttons we can still use
 * @param buttons - Array of which targets each button affects
 */
function findMinimumPresses(targets: number[], availableButtons: number, buttons: number[][]): number {
    // BASE CASE: All targets satisfied
    if (targets.every(t => t === 0)) {
        return 0;
    }
    
    // STEP 1: Choose the "hardest" target to satisfy
    // (the one with fewest available buttons that can help it)
    const chosenTarget = chooseHardestTarget(targets, availableButtons, buttons);
    
    if (chosenTarget === -1) {
        return Infinity; // No solution possible
    }
    
    const targetValue = targets[chosenTarget.index];
    const helpfulButtons = chosenTarget.buttons;
    
    // STEP 2: Remove these helpful buttons from future consideration
    // (once we decide how to use them for this target, they're "spent")
    let newAvailableButtons = availableButtons;
    for (const buttonIdx of helpfulButtons) {
        newAvailableButtons = removeButton(newAvailableButtons, buttonIdx);
    }
    
    // STEP 3: Try all possible ways to press these buttons to satisfy the target
    let bestResult = Infinity;
    
    const allWays = findAllWaysToSum(helpfulButtons.length, targetValue);
    
    for (const way of allWays) {
        // Check if this way of pressing buttons is valid
        const newTargets = applyButtonPresses(targets, way, helpfulButtons, buttons);
        
        if (newTargets !== null) { // Valid application
            const totalPresses = way.reduce((sum, presses) => sum + presses, 0);
            const futureResult = findMinimumPresses(newTargets, newAvailableButtons, buttons);
            
            if (futureResult !== Infinity) {
                bestResult = Math.min(bestResult, totalPresses + futureResult);
            }
        }
    }
    
    return bestResult;
}

/**
 * Choose the target that's hardest to satisfy (has fewest helpful buttons)
 * This is the key optimization - by solving hard targets first, we prune more branches
 */
function chooseHardestTarget(targets: number[], availableButtons: number, buttons: number[][]) {
    let bestTarget = { index: -1, buttons: [] as number[], buttonCount: Infinity };
    
    for (let i = 0; i < targets.length; i++) {
        if (targets[i] <= 0) continue; // Already satisfied
        
        // Find which available buttons can help this target
        const helpfulButtons = [];
        for (let j = 0; j < buttons.length; j++) {
            if (isButtonAvailable(availableButtons, j) && buttons[j].includes(i)) {
                helpfulButtons.push(j);
            }
        }
        
        // Choose target with fewest helpful buttons (hardest to satisfy)
        if (helpfulButtons.length > 0 && helpfulButtons.length < bestTarget.buttonCount) {
            bestTarget = {
                index: i,
                buttons: helpfulButtons,
                buttonCount: helpfulButtons.length
            };
        }
    }
    
    return bestTarget.index === -1 ? -1 : bestTarget;
}

/**
 * Find all ways to distribute 'total' presses among 'numButtons' buttons
 * For example: distribute 5 presses among 2 buttons gives:
 * [0,5], [1,4], [2,3], [3,2], [4,1], [5,0]
 */
function findAllWaysToSum(numButtons: number, total: number): number[][] {
    const ways: number[][] = [];
    
    function generateWays(buttonIndex: number, currentWay: number[], remaining: number) {
        if (buttonIndex === numButtons) {
            if (remaining === 0) {
                ways.push([...currentWay]);
            }
            return;
        }
        
        // Try giving 0 to 'remaining' presses to this button
        for (let presses = 0; presses <= remaining; presses++) {
            currentWay[buttonIndex] = presses;
            generateWays(buttonIndex + 1, currentWay, remaining - presses);
        }
    }
    
    generateWays(0, new Array(numButtons).fill(0), total);
    return ways;
}

/**
 * Apply button presses and return new target state, or null if invalid
 */
function applyButtonPresses(
    targets: number[], 
    presses: number[], 
    buttonIndices: number[], 
    buttons: number[][]
): number[] | null {
    const newTargets = [...targets];
    
    // Apply each button press
    for (let i = 0; i < presses.length; i++) {
        const buttonIdx = buttonIndices[i];
        const numPresses = presses[i];
        
        if (numPresses > 0) {
            // This button affects multiple targets
            for (const targetIdx of buttons[buttonIdx]) {
                if (targetIdx < newTargets.length) {
                    newTargets[targetIdx] -= numPresses;
                    
                    // CRITICAL: If we go below 0, this is invalid
                    // (can't "undo" button presses)
                    if (newTargets[targetIdx] < 0) {
                        return null;
                    }
                }
            }
        }
    }
    
    return newTargets;
}

/**
 * Helper functions for bitmask operations
 */
function isButtonAvailable(mask: number, buttonIndex: number): boolean {
    return (mask & (1 << buttonIndex)) !== 0;
}

function removeButton(mask: number, buttonIndex: number): number {
    return mask & ~(1 << buttonIndex);
}

// Main execution
let totalSum = 0;
let solutionsFound = 0;

for (let lineNumber = 0; lineNumber < lines.length; lineNumber++) {
    const line = lines[lineNumber];
    if (!line.trim()) continue;
    
    // Parse input line
    const buttonMatches = line.match(/\(([^)]+)\)/g);
    const buttons: number[][] = [];
    
    if (buttonMatches) {
        for (const match of buttonMatches) {
            const numbersStr = match.slice(1, -1);
            if (numbersStr.trim()) {
                const numbers = numbersStr.split(',').map(n => parseInt(n.trim()));
                buttons.push(numbers);
            }
        }
    }
    
    const targetMatch = line.match(/\{([^}]+)\}/);
    const target: number[] = [];
    
    if (targetMatch) {
        const numbersStr = targetMatch[1];
        if (numbersStr.trim()) {
            target.push(...numbersStr.split(',').map(n => parseInt(n.trim())));
        }
    }
    
    // Solve the problem
    const result = solve(buttons, target);
    
    if (result !== null) {
        if (lineNumber < 5) {
            console.log(`Line ${lineNumber + 1}: ${result} presses`);
        }
        totalSum += result;
        solutionsFound++;
    } else {
        if (lineNumber < 10) {
            console.log(`Line ${lineNumber + 1}: No solution found`);
        }
    }
    
    if ((lineNumber + 1) % 25 === 0) {
        console.log(`Processed ${lineNumber + 1} lines, found ${solutionsFound} solutions`);
    }
}

console.log(`\nFinal Results:`);
console.log(`- Processed ${lines.length} lines`);
console.log(`- Found solutions for ${solutionsFound} lines`);
console.log(`- Total button presses needed: ${totalSum}`);