class Game:
    def __init__(self, a, b, prize):
        self.cost_a = 3
        self.cost_b = 1
        self.offset = 10000000000000
        self.a = a
        self.b = b
        self.prize = (prize[0]+self.offset, prize[1]+self.offset)
    
    def calculate_minimum_cost(self):
        # First solve equation
        px, py = self.prize
        ax, ay = self.a
        bx, by = self.b
        B = (ax*py - ay*px) / (ax*by - ay*bx)
        A = ((px - bx*B) / ax)
        print("A: ", A, " B: ", B)
        return A*self.cost_a + B*self.cost_b
    

games = []

with open("input.txt") as f:
    content = [i.strip() for i in f.readlines()]
    i = 0
    while i < len(content):
        buttonA = content[i].split(" ")
        a = (int(buttonA[2][1:len(buttonA[2])-1]), int(buttonA[3][1:]))
        buttonB = content[i+1].split(" ")
        b = (int(buttonB[2][1:len(buttonB[2])-1]), int(buttonB[3][1:]))
        prizeS = content[i+2].split(" ")
        prize = (int(prizeS[1][2:len(prizeS[1])-1]), int(prizeS[2][2:]))
        games.append(Game(a, b, prize))
        i+=4
    nr = 1
    tokens = 0
    for game in games:
        res = game.calculate_minimum_cost()
        print(nr, " ", res)
        nr += 1
        if res == int(res):
            tokens += int(res)
    print(tokens)
    

# px = Aax + Bbx -> (px - bxB) / ax = A
# py = Aay + Bby -> axpy - aypx / byax-aybx = B
# 8400 = a*94 + b*22
# 5400 = a*34 + b*67
