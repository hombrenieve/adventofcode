import sys

# for each line passed to this function
# convert each sequence to numbers, they are separated by blanks
# check for the highest and lowest number in the sequence
# and return their difference
def check_line(line):
    numbers = [int(i) for i in line]
    numbers.sort()
    return numbers[len(numbers)-1] - numbers[0]

accumulator = 0
for line in sys.stdin:
    accumulator += check_line(line.split())

print(accumulator)
