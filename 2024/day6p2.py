class LoopException(Exception):
    pass


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
                        self.init = self.marker
                        self.direction = 0
                        self.content[i][c] = 'X'
                        break
            self.direction_change_counter = 0
    
    def new_coord(self):
        return self.marker[0] + self.directions[self.direction][0], self.marker[1] + self.directions[self.direction][1]

    def move(self):
        new_c = self.new_coord()
        if new_c[0] < 0 or new_c[0] >= len(self.content[0]) or new_c[1] < 0 or new_c[1] >= len(self.content):
            return False
        match self.content[new_c[1]][new_c[0]]:
            case '.':
                self.content[new_c[1]][new_c[0]] = 'X'
                self.direction_change_counter = 0
                self.marker = new_c
            case '#':
                    # change direction
                    self.direction = (self.direction + 1) % 4
                    self.direction_change_counter += 1
                    if self.direction_change_counter == 4:
                        raise LoopException("Loop detected")
                    return self.move()
            case 'X':
                    self.marker = new_c
            case _:
                print("Unexpected character: " + self.content[new_c[1]][new_c[0]])
                exit(-1)
        return True
    
    def set_block(self, pos):
        self.content[pos[1]][pos[0]] = '#'
    
    def remove_block(self, pos):
        self.content[pos[1]][pos[0]] = '.'
    
    def clean(self):
        for i in range(len(self.content)):
            for c in range(len(self.content[i])):
                if self.content[i][c] == 'X':
                    self.content[i][c] = '.'
        self.marker = self.init
        self.direction = 0
        self.content[self.init[1]][self.init[0]] = 'X'
    
    def calculate_possible_loops(self):
        res = 0
        for i in range(len(self.content)):
            for c in range(len(self.content[i])):
                if self.content[i][c] == '.':
                    self.set_block((c, i))
                    try:
                        while self.move():
                            pass
                    except LoopException:
                        print("Loop detected with block at " + str((c, i)))
                        res += 1
                    self.remove_block((c, i))
                    self.clean()
        return res
    
if __name__ == "__main__":
    puzzle = Puzzle()
    print(puzzle.calculate_possible_loops())
    
