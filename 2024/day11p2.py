
def solution_util(element, iterations, memo):
    if iterations == 0:
        return 1
    if (element, iterations) in memo:
        return memo[(element, iterations)]
    if element == 0:
        memo[(element, iterations)] = solution_util(1, iterations - 1, memo)
        return memo[(element, iterations)]
    strel = str(element)
    size = len(strel)
    if size % 2 == 0:
        # split in half
        half = int(size // 2)
        el1 = int(strel[:half])
        el2 = int(strel[half:])
        memo[(element, iterations)] = solution_util(el1, iterations - 1, memo) + solution_util(el2, iterations - 1, memo)
        return memo[(element, iterations)]
    memo[(element, iterations)] = solution_util(element * 2024, iterations - 1, memo)
    return memo[(element, iterations)]

def solution(element, iterations):
    memo = {}
    return solution_util(element, iterations, memo)


with open("input.txt") as f:
    content = [int(x) for x in f.readline().strip().split(" ")]
    print(sum(solution(x, 75) for x in content))
