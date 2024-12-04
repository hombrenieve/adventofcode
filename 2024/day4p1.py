
WORD = "XMAS"

class WS:
    def __init__(self):
        # read from input.txt in bidimensional array of chars
        with open('input.txt') as f:
            self.content = [i.strip() for i in f.readlines()]
    
    def take_snippet(self, position, direction):
        snippet = ""
        for wc in range(len(WORD)):
            # check inbounds
            newpos = position[0]+wc*direction[0], position[1]+wc*direction[1]
            if newpos[0] < 0 or newpos[0] >= len(self.content) or newpos[1] < 0 or newpos[1] >= len(self.content[0]):
                return None
            snippet += str(self.content[newpos[0]][newpos[1]])
        return snippet

    def find_word(self):
        occurrences = 0
        for i in range(len(self.content)):
            for j in range(len(self.content[0])):
                if self.content[i][j] == WORD[0]:
                    for d in [ (0, -1), (0, 1), (-1, 0), (1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1) ]:
                            snippet = self.take_snippet((i, j), d)
                            if snippet is None:
                                continue
                            if snippet == WORD:
                                occurrences += 1
        return occurrences


if __name__ == "__main__":
    ws = WS()
    print(ws.find_word())