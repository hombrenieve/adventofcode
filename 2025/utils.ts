import { readFileSync } from 'fs';
import { join } from 'path';

export function readInput(filename = 'input.txt'): string[] {
  try {
    const input = readFileSync(join(__dirname, filename), 'utf-8').trim();
    return input.split('\n');
  } catch (error) {
    console.log(`No ${filename} found - create one in the same folder`);
    return [];
  }
}