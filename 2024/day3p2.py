import re

# read each line from a file input.txt
with open('input.txt') as f:
    content = "".join([i.strip() for i in f.readlines()])
    m = re.findall('mul\(\d+,\d+\)|do\(\)|don\'t\(\)', content)
    enabled = True
    res = 0
    for el in m:
        if enabled:
            op = re.search('mul\((\d+),(\d+)\)', el)
            if op:
                res += int(op.group(1)) * int(op.group(2))
        if re.search('do\(\)', el):
            enabled = True
        if re.search('don\'t\(\)', el):
            enabled = False
    print(res)
    