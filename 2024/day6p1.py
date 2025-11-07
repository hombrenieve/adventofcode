
class Puzzle:

    def __init__(self):
        self.directions = [ (0, -1), (1, 0), (0, 1), (-1, 0)]
        with open('input.txt') as f:
            self.content = [i.strip() for i in f.readlines()]
            # convert to char array
            self.content = [[c for c in line] for line in self.content]
            # find marker
            for i in range(len(self.content)):
                for c in range(len(self.content[i])):
                    if self.content[i][c] == '^':
                        self.marker = (c, i)
                        self.direction = 0
                        self.content[i][c] = 'X'
                        self.visited = 1
                        break
    
    def new_coord(self):
        return self.marker[0] + self.directions[self.direction][0], self.marker[1] + self.directions[self.direction][1]

    def move(self):
        new_c = self.new_coord()
        if new_c[0] < 0 or new_c[0] >= len(self.content[0]) or new_c[1] < 0 or new_c[1] >= len(self.content):
            return False
        match self.content[new_c[1]][new_c[0]]:
            case '.':
                self.content[new_c[1]][new_c[0]] = 'X'
                self.visited += 1
                self.marker = new_c
            case '#':
                    # change direction
                    self.direction = (self.direction + 1) % 4
                    return self.move()
            case 'X':
                    self.marker = new_c
            case _:
                print("Unexpected character: " + self.content[new_c[1]][new_c[0]])
                exit(-1)
        return True
    
    def run_puzzle(self):
        while self.move():
            pass
        return self.visited
    
if __name__ == "__main__":
    puzzle = Puzzle()
    print(puzzle.run_puzzle())
    
