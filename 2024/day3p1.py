import re

# read each line from a file input.txt
with open('input') as f:
    content = "".join([i.strip() for i in f.readlines()])
    m = re.findall('mul\(\d+,\d+\)', content)
    print(m)
    res = 0
    for el in m:
        op = re.search('mul\((\d+),(\d+)\)', el)
        res += int(op.group(1)) * int(op.group(2))
    print(res)