
def solution(element, iterations):
    if iterations == 0:
        return 1
    if element == 0:
        return solution(1, iterations - 1)
    strel = str(element)
    size = len(strel)
    if size % 2 == 0:
        # split in half
        half = int(size // 2)
        return solution(int(strel[:half]), iterations - 1) + solution(int(strel[half:]), iterations - 1)
    return solution(element * 2024, iterations - 1)


with open("inputEx.txt") as f:
    content = [int(x) for x in f.readline().strip().split(" ")]
    print(sum(solution(x, 25) for x in content))
