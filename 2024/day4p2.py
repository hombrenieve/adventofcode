
class WS:
    def __init__(self):
        # read from input.txt in bidimensional array of chars
        with open('input.txt') as f:
            self.content = [i.strip() for i in f.readlines()]
    
    def get_value_in(self, position):
        i, j = position
        if i < 0 or i >= len(self.content) or j < 0 or j >= len(self.content[0]):
            return "."
        return self.content[i][j]

    def check_x_mas(self, position):
        i, j = position
        fx = (self.get_value_in((i-1, j-1)), self.get_value_in((i+1, j+1)))
        sx = (self.get_value_in((i-1, j+1)), self.get_value_in((i+1, j-1)))
        ft = (fx[0] == "M" and fx[1] == "S") or (fx[0] == "S" and fx[1] == "M")
        st = (sx[0] == "M" and sx[1] == "S") or (sx[0] == "S" and sx[1] == "M")
        return ft and st
        
    def find_x(self):
        res = 0
        for i in range(len(self.content)):
            for j in range(len(self.content[0])):
                if self.content[i][j] == 'A':
                    if self.check_x_mas((i, j)):
                        res += 1
        return res


if __name__ == "__main__":
    ws = WS()
    print(ws.find_x())