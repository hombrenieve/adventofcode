# receive a list of numbers from stdin
# output the missing numbers, considering that
# the list can have any number of elements and is unordered

import sys

for line in sys.stdin:
    numbers = sorted(set(int(i) for i in line.split()))
    # Generate a list of all numbers from the smallest to the largest in the input list
    all_numbers = list(range(numbers[0], numbers[len(numbers)-1]+1))
    # Find the missing numbers
    missing = [i for i in all_numbers if i not in numbers]
    print(' '.join(map(str, missing)))