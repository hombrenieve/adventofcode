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

# calculate the score number by multiplying each number on list 1 by the number
# of times it appears in list 2
list_scores = [x * list2.count(x) for x in list1]

# calculate the sum of the scores
print(sum(list_scores))