from itertools import chain

def apply_rules(element):
    if element == 0:
        return [1]
    strel = str(element)
    size = len(strel)
    if size % 2 == 0:
        # split in half
        half = int(size // 2)
        return [int(strel[:half]), int(strel[half:])]
    return [element * 2024]

def transform(content):
    res = [apply_rules(x) for x in content]
    return list(chain.from_iterable(res))



with open("input.txt") as f:
    content = [int(x) for x in f.readline().strip().split(" ")]
    for i in range(75):
        print(i)
        content = transform(content)
    print(len(content))
