class Direction:
    def __init__(self):
        self.directions = [ (0, 1), (-1, 0), (0, -1), (1, 0)]
        self.current = 1
    
    def change_direction(self):
        self.current = (self.current + 1) % 4
    
    def get_next(self):
        return self.directions[(self.current + 1) % 4]
    
    def get_direction(self):
        return self.directions[self.current]

class Matrix:
    def __init__(self, w, h):
        self.w = w
        self.h = h
        self.matrix = [[0 for x in range(w)] for y in range(h)] 

    def move(position, direction):
        return (position[0]+direction[0], position[1]+direction[1])

    def valid(self, position):
        return position[0] >= 0 and position[1] >= 0 and position[0] < len(self.matrix) and position[1] < len(self.matrix[0]) and self.matrix[position[0]][position[1]] == 0

    def sum_neighbours(self, position):
        res = 0
        x, y = position
        for i in range(-1, 2):
            for j in range(-1, 2):
                res += self.matrix[x+i][y+j]
        return res
    
    def set_value(self, position, value):
        self.matrix[position[0]][position[1]] = value

    def print(self):
        for i in range(len(self.matrix)-1, -1, -1):
            for j in range(len(self.matrix[0])-1, -1, -1):
                print(self.matrix[i][j], end=" ")
            print()
    
    def spiral_run(self, orig, target):
        pos = orig
        direction = Direction()
        res = 0
        while res <= target:
            wanted_direction = direction.get_next()
            if self.valid(Matrix.move(pos, wanted_direction)):
                direction.change_direction()
                pos = Matrix.move(pos, direction.get_direction())
            elif self.valid(Matrix.move(pos, direction.get_direction())):
                pos = Matrix.move(pos, direction.get_direction())
            else:
                self.print()
                print("No valid direction!")
                exit(-1)
            res = self.sum_neighbours(pos)
            self.set_value(pos, res)
        return res

if __name__ == "__main__":
    matrix = Matrix(10, 10)
    orig = (5, 5)
    matrix.set_value(orig, 1)
    res = matrix.spiral_run(orig, 806)
    print(res)