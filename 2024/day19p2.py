

class StripeAnalyzer:

    def __init__(self, stripes):
        self.stripes = stripes
        self.coefs = {s: 1 for s in self.stripes}
        self.maxstripe = 0
        # calculate coeficients of each stripe
        for i in range(len(self.stripes)):
            stripes = [s for s in self.stripes if s != self.stripes[i]]
            self.coefs[self.stripes[i]] = 1 +StripeAnalyzer.calculate_coefficient(self.stripes[i], stripes)
            self.maxstripe = max(self.maxstripe, len(self.stripes[i]))
        
    def count_possible_stripes(self, towel):
        coef = 1
        t = str(towel)
        found = True
        while t != '' and found:
            found = False
            for i in range(self.maxstripe, 0, -1):
                current = t[:i]
                if current in self.stripes:
                    coef *= self.coefs[current]
                    last = len(t)
                    t = t[i:]
                    found = True                    
                    break
        if t == '':
            return coef
        return 0  

    def get_possible_stripes(stripe, stripes):
        res = []
        for s in stripes:
            if stripe.startswith(s):
                res.append(s)
        return res

    def calculate_coefficient(stripe, stripes):
        if stripe == '':
            return 1
        possible = StripeAnalyzer.get_possible_stripes(stripe, stripes)
        res = 0
        for s in possible:
            res += StripeAnalyzer.calculate_coefficient(stripe[len(s):], stripes)
        return res



with open('inputEx.txt') as f:
    content = [i.strip() for i in f.readlines()]
    analyzer = StripeAnalyzer(content[0].split(', '))
    towels = content[2:]
    count = 0
    order = 0
    for towel in towels:
        res = analyzer.count_possible_stripes(towel)
        count += res
        order += 1
        print(order, ": ", res)
    print(count)

    