
class Test:
    def __init__(self, definition):
        def_split = definition.split(": ")
        self.result = int(def_split[0])
        self.operands = [int(x) for x in def_split[1].split(" ")]

    def check(self, operands, acc):
        if acc > self.result:
            return False
        if len(operands) == 0:
            return acc == self.result
        else:
            res = self.check(operands[1:], acc + operands[0])
            if not res:
                res = self.check(operands[1:], acc * operands[0])
            return res
    
    def get_result(self):
        return self.result if self.check(self.operands, 0) else 0

tests = []

with open("input.txt") as f:
    content = f.readlines()
    content = [x.strip() for x in content]
    for definition in content:
        tests.append(Test(definition))
    print(sum(t.get_result() for t in tests))