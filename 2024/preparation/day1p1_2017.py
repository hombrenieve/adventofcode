# read a sequence of numbers from stdin in one line
# convert each char to int and check if it is equal to the next in sequence
# the last char must be checked against the first
# if it is, add it to the sum
# print the sum


import sys


def check_line(line):
    numbers = [int(i) for i in line]
    sum = 0
    for i in range(1, len(numbers)):
        if numbers[i] == numbers[i-1]:
            sum += numbers[i]
    if numbers[len(numbers)-1] == numbers[0]:
        sum += numbers[0]
    return sum

for line in sys.stdin:
    print(check_line(line.strip()))
