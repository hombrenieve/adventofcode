
class Rules():
    def __init__(self):
        self.rules = []

    def append(self, n1, n2):
        self.rules.append((n1, n2))

    def check(self, line):
        for r in self.rules:
            try:
                f = line.index(r[0])
                s = line.index(r[1])
                if s < f:
                    return False
            except ValueError:
                continue
        return True
    
def get_middle_value(line):
    return line[len(line)//2]

with open("input.txt") as f:
    content = f.readlines()
    content = [x.strip() for x in content]
    rules = Rules()
    mark = 0
    for i in range(len(content)):
        if content[i] == "":
            mark = i+1
            break
        r = content[i].split("|")
        rules.append(int(r[0]), int(r[1]))

    # check
    res = 0
    for i in range(mark, len(content)):
        sp = content[i].split(",")
        line = [int(x) for x in sp]
        if rules.check(line):
            res += get_middle_value(line)
    
    print(res)

