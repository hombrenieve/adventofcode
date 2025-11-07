class StripeAnalyzer:

    def __init__(self, stripes):
        self.stripes = stripes
    
    def get_possible_stripes(self, towel):
        res = []
        for s in self.stripes:
            if towel.startswith(s):
                res.append(s)
        return res

    def can_make_towel(self, towel):
        if towel in self.stripes:
            return True
        possible = self.get_possible_stripes(towel)
        for s in possible:
            res = self.can_make_towel(towel[len(s):])
            if res:
                return True
        return False



with open('input.txt') as f:
    content = [i.strip() for i in f.readlines()]
    analyzer = StripeAnalyzer(content[0].split(', '))
    towels = content[2:]
    count = 0
    for towel in towels:
        if analyzer.can_make_towel(towel):
            count += 1
    print(count)

    