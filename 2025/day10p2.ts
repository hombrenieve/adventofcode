import { readInput } from './utils';
import { Worker, isMainThread, parentPort, workerData } from 'worker_threads';
import { cpus } from 'os';

console.log("Day 10 - Part 2 (Multi-threaded FIFO)");

if (isMainThread) {
    // MAIN THREAD: Coordinate the work with FIFO queue
    main();
} else {
    // WORKER THREAD: Process tasks from queue
    worker();
}

async function main() {
    const lines = readInput('inputOf.txt');
    console.log(`Read ${lines.length} lines from input file`);
    
    const numCPUs = cpus().length;
    const numWorkers = Math.max(1, numCPUs - 1);
    console.log(`Using ${numWorkers} worker threads (${numCPUs} CPUs available)`);
    
    // Create task queue - each task is a line with its index
    const taskQueue: Array<{ lineNumber: number, line: string }> = [];
    for (let i = 0; i < lines.length; i++) {
        if (lines[i].trim()) {
            taskQueue.push({ lineNumber: i, line: lines[i] });
        }
    }
    
    console.log(`Created task queue with ${taskQueue.length} tasks`);
    
    // Shared state
    let completedTasks = 0;
    let currentTaskIndex = 0;
    const results: Array<{ lineNumber: number, result: number | null }> = [];
    const workers: Worker[] = [];
    
    const startTime = Date.now();
    
    // Create workers
    const workerPromises = [];
    
    for (let i = 0; i < numWorkers; i++) {
        const workerPromise = new Promise<void>((resolve, reject) => {
            const worker = new Worker(`
                const { register } = require('ts-node');
                register({
                    transpileOnly: true,
                    compilerOptions: {
                        module: 'commonjs',
                        target: 'es2020'
                    }
                });
                require('${__filename}');
            `, {
                eval: true,
                workerData: { 
                    workerId: i + 1
                }
            });
            
            // Handle messages from worker
            worker.on('message', (message) => {
                if (message.type === 'requestTask') {
                    // Worker is requesting a new task
                    if (currentTaskIndex < taskQueue.length) {
                        const task = taskQueue[currentTaskIndex];
                        currentTaskIndex++;
                        
                        console.log(`🔄 Worker ${message.workerId} starting line ${task.lineNumber + 1} (${currentTaskIndex}/${taskQueue.length})`);
                        
                        worker.postMessage({
                            type: 'task',
                            task: task
                        });
                    } else {
                        // No more tasks
                        worker.postMessage({
                            type: 'noMoreTasks'
                        });
                    }
                } else if (message.type === 'taskComplete') {
                    // Worker completed a task
                    const { lineNumber, result, processingTime } = message;
                    results.push({ lineNumber, result });
                    completedTasks++;
                    
                    const timeStr = processingTime > 1000 ? 
                        `${(processingTime / 1000).toFixed(1)}s` : 
                        `${processingTime}ms`;
                    
                    const resultStr = result !== null ? `${result} presses` : 'no solution';
                    console.log(`✅ Worker ${message.workerId} completed line ${lineNumber + 1} in ${timeStr} (${resultStr}) - ${completedTasks}/${taskQueue.length} done`);
                    
                } else if (message.type === 'workerFinished') {
                    // Worker has no more tasks and is finishing
                    console.log(`🏁 Worker ${message.workerId} finished`);
                    resolve();
                }
            });
            
            worker.on('error', (error) => {
                console.error(`❌ Worker ${i + 1} error:`, error);
                reject(error);
            });
            
            worker.on('exit', (code) => {
                if (code !== 0) {
                    console.error(`❌ Worker ${i + 1} exited with code ${code}`);
                }
            });
            
            workers.push(worker);
        });
        
        workerPromises.push(workerPromise);
    }
    
    console.log(`🚀 Started ${numWorkers} workers with FIFO task queue`);
    
    // Wait for all workers to complete
    try {
        await Promise.all(workerPromises);
        
        const endTime = Date.now();
        console.log(`\n🎉 All workers completed in ${(endTime - startTime) / 1000}s`);
        
        // Sort results by line number
        results.sort((a, b) => a.lineNumber - b.lineNumber);
        
        // Calculate final statistics
        let totalSum = 0;
        let solutionsFound = 0;
        
        for (const { lineNumber, result } of results) {
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
        }
        
        console.log(`\n📊 Final Results:`);
        console.log(`- Processed ${taskQueue.length} lines`);
        console.log(`- Found solutions for ${solutionsFound} lines (${(solutionsFound/taskQueue.length*100).toFixed(1)}%)`);
        console.log(`- Total button presses needed: ${totalSum}`);
        console.log(`- Total processing time: ${(endTime - startTime) / 1000}s`);
        console.log(`- Average time per line: ${((endTime - startTime) / taskQueue.length / 1000).toFixed(2)}s`);
        
    } catch (error) {
        console.error('❌ Error in worker coordination:', error);
    } finally {
        // Clean up workers
        workers.forEach(worker => worker.terminate());
    }
}

function worker() {
    const { workerId } = workerData;
    
    // Request first task
    parentPort?.postMessage({
        type: 'requestTask',
        workerId: workerId
    });
    
    // Handle messages from main thread
    parentPort?.on('message', (message) => {
        if (message.type === 'task') {
            // Process the assigned task
            const { lineNumber, line } = message.task;
            processLine(lineNumber, line, workerId);
            
        } else if (message.type === 'noMoreTasks') {
            // No more tasks available, finish
            parentPort?.postMessage({
                type: 'workerFinished',
                workerId: workerId
            });
        }
    });
}

function processLine(lineNumber: number, line: string, workerId: number) {
    const startTime = Date.now();
    
    // Parse input line
    const buttonMatches = line.match(/\(([^)]+)\)/g);
    const buttons: number[][] = [];
    
    if (buttonMatches) {
        for (const match of buttonMatches) {
            const numbersStr = match.slice(1, -1);
            if (numbersStr.trim()) {
                const numbers = numbersStr.split(',').map((n: string) => parseInt(n.trim()));
                buttons.push(numbers);
            }
        }
    }
    
    const targetMatch = line.match(/\{([^}]+)\}/);
    const target: number[] = [];
    
    if (targetMatch) {
        const numbersStr = targetMatch[1];
        if (numbersStr.trim()) {
            target.push(...numbersStr.split(',').map((n: string) => parseInt(n.trim())));
        }
    }
    
    // Solve the problem
    const result = solve(buttons, target);
    const endTime = Date.now();
    const processingTime = endTime - startTime;
    
    // Send result back to main thread
    parentPort?.postMessage({
        type: 'taskComplete',
        workerId: workerId,
        lineNumber: lineNumber,
        result: result,
        processingTime: processingTime
    });
    
    // Request next task
    parentPort?.postMessage({
        type: 'requestTask',
        workerId: workerId
    });
}

/**
 * Main solver function - tries to find minimum button presses to reach exact targets
 */
function solve(buttons: number[][], target: number[]): number | null {
    const remainingTargets = [...target];
    const allButtonsAvailable = (1 << buttons.length) - 1;
    
    const result = findMinimumPresses(remainingTargets, allButtonsAvailable, buttons);
    
    return result === Infinity ? null : result;
}

/**
 * Core recursive function that finds minimum button presses
 */
function findMinimumPresses(targets: number[], availableButtons: number, buttons: number[][]): number {
    // BASE CASE: All targets satisfied
    if (targets.every(t => t === 0)) {
        return 0;
    }
    
    // STEP 1: Choose the "hardest" target to satisfy
    const chosenTarget = chooseHardestTarget(targets, availableButtons, buttons);
    
    if (chosenTarget === -1) {
        return Infinity; // No solution possible
    }
    
    const targetValue = targets[chosenTarget.index];
    const helpfulButtons = chosenTarget.buttons;
    
    // STEP 2: Remove these helpful buttons from future consideration
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