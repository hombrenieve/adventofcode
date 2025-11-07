import sys

# for each line passed to this function
# convert each element into integer numbers
# and find the 2 numbers that are evenly divisible
# and return their quotient
def check_line(line):
    numbers = [int(i) for i in line]
    for i in range(0, len(numbers)):
        for j in range(i + 1, len(numbers)):
            if numbers[i] % numbers[j] == 0:
                return numbers[i] // numbers[j]
            else:
                if numbers[j] % numbers[i] == 0:
                    return numbers[j] // numbers[i]
    return 0

accumulator = 0
for line in sys.stdin:
    accumulator += check_line(line.split())

print(accumulator)
