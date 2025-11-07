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
        if direction[0] == 0:
            next = self.table[self.position[0] + direction[0]][self.position[1] + direction[1]]
            if next.push(self.type, direction):
                self.type = by
                return True
        else:
            next = self.table[self.position[0] + direction[0]][self.position[1] + direction[1]]
            next_side = None
            self_side = None
            if self.type == '[':
                next_side = self.table[next.position[0]][next.position[1] + 1]
                self_side = self.table[self.position[0]][self.position[1] + 1]
            if self.type == ']':
                next_side = self.table[next.position[0]][next.position[1] - 1]
                self_side = self.table[self.position[0]][self.position[1] - 1]
            if next_side.push(self_side.type, direction) and next.push(self.type, direction):
                self.type = by
                self_side.type = '.'
                return True
        return False
    
    def try_push(self, direction):
        if self.type == '#':
            return False
        if self.type == '.':
            return True
        if direction[0] == 0:
            next = self.table[self.position[0] + direction[0]][self.position[1] + direction[1]]
            return next.try_push(direction)
        else:
            next = self.table[self.position[0] + direction[0]][self.position[1] + direction[1]]
            next_side = None
            if self.type == '[':
                next_side = self.table[next.position[0]][next.position[1] + 1]
            if self.type == ']':
                next_side = self.table[next.position[0]][next.position[1] - 1]
            return next_side.try_push(direction) and next.try_push(direction)
    
    def __repr__(self):
        return str(self.type)

def calculate_gps(table):
    res = 0
    for i in range(len(table)):
        for c in range(len(table[i])):
            if table[i][c].type == '[':
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
            if content[i][c] == 'O':
                table[i].append(Element((i, 2*c), '[', table))
                table[i].append(Element((i, 2*c+1), ']', table))
                continue
            if content[i][c] == '@':
                start = (i, 2*c)
                table[i].append(Element((i, 2*c), '@', table))
                table[i].append(Element((i, 2*c+1), '.', table))
                continue
            table[i].append(Element((i, 2*c), content[i][c], table))
            table[i].append(Element((i, 2*c+1), content[i][c], table))
        i += 1
    
    i += 1
    pos = start
    while i < len(content):
        for c in range(len(content[i])):
            dir = Direction(content[i][c])
            pos_to_push = (pos[0] + dir.direction[0], pos[1] + dir.direction[1])
            if table[pos_to_push[0]][pos_to_push[1]].try_push(dir.direction):
                table[pos_to_push[0]][pos_to_push[1]].push('@', dir.direction)
                table[pos[0]][pos[1]].type = '.'
                pos = pos_to_push
        i += 1
    
print(calculate_gps(table))