# read a sequence of numbers from stdin in one line
# convert each char to int and check if it is equal to the next in sequence
# the last char must be checked against the first
# if it is, add it to the sum
# print the sum


import sys


def check_line(line):
    numbers = [int(i) for i in line]
    sum = 0
    check_step = len(numbers) // 2
    for i in range(0, len(numbers)):
        if numbers[i] == numbers[(i + check_step) % len(line)]:
            sum += numbers[i]
    return sum

for line in sys.stdin:
    print(check_line(line.strip()))
