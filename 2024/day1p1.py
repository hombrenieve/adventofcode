# read from stdin each line containing 2 numbers
# store each of them in separated lists

import sys

list1 = []
list2 =[]

for line in sys.stdin:
    numbers = line.split()
    list1.append(int(numbers[0]))
    list2.append(int(numbers[1]))

# sort ascendly both lists
list1.sort()
list2.sort()

# calculate the difference between each number on the lists
list_distances = [abs(x - y) for x, y in zip(list1, list2)]

# calculate the sum of the differences
print(sum(list_distances))