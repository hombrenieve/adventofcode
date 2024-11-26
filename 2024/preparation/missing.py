# receive a list of numbers from stdin
# output the missing numbers, considering that
# the list can have any number of elements and is unordered

import sys

for line in sys.stdin:
    numbers = [int(i) for i in line.split()]
    # remove duplicates and sort
    numbers = list(set(numbers))
    numbers.sort()
    all_numbers = [i for i in range(0, numbers[len(numbers)-1]+1)]
    missing = [i for i in all_numbers if i not in numbers]
    print(','.join(map(str, missing)))