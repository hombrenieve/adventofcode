class Direction:
    def __init__(self, character):
        match character:
            case '<':
                self.direction = (0, -1)
            case '>':
                self.direction = (0, 1)
            case '^':
                self.direction = (-1, 0)
            case 'v':
                self.direction = (1, 0)
            case _:
                print("Unexpected character: " + character)
                exit(-1)

class Element:
    def __init__(self, pos, type, table):
        self.position = pos
        self.type = type
        self.table = table
    
    def push(self, by, direction):
        if self.type == '#':
            return False
        if self.type == '.':
            self.type = by
            return True
        next = self.table[self.position[0] + direction[0]][self.position[1] + direction[1]]
        if next.push(self.type, direction):
            self.type = by
            return True
        return False
    
    def __repr__(self):
        return str(self.type)

def calculate_gps(table):
    res = 0
    for i in range(len(table)):
        for c in range(len(table[i])):
            if table[i][c].type == 'O':
                box = (i, c)
                res += 100*box[0] + box[1]
    return res

with open("input.txt") as f:
    content = f.readlines()
    content = [[c for c in line.strip()] for line in content]
    table = []
    i = 0
    while len(content[i]) != 0:
        table.append([])
        for c in range(len(content[i])):
            table[i].append(Element((i, c), content[i][c], table))
            if content[i][c] == '@':
                start = (i, c)
        i += 1
    
    i += 1
    pos = start
    while i < len(content):
        for c in range(len(content[i])):
            dir = Direction(content[i][c])
            pos_to_push = (pos[0] + dir.direction[0], pos[1] + dir.direction[1])
            if table[pos_to_push[0]][pos_to_push[1]].push('@', dir.direction):
                table[pos[0]][pos[1]].type = '.'
                pos = pos_to_push
        i += 1
    
print(calculate_gps(table))