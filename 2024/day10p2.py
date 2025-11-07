score = 0

class Node:
    def __init__(self, value):
        self.value = value
        self.score = -1
        self.visited = False
    
    def set_table_and_position(self, table, position):
        self.table = table
        self.position = position

    def get_value(self):
        return self.value
    
    def get_score(self):
        return self.score
    
    def is_next_step(self, node):
        return node.value - self.value == 1
    
    def calculate_score(self):
        global score
        if self.value == 9:
            score += 1
            return 1
        for move in [(-1,0), (1,0), (0,-1), (0,1)]:
            new_coord = (self.position[0] + move[0], self.position[1] + move[1])
            if new_coord[0] < 0 or new_coord[0] >= len(self.table[0]) or new_coord[1] < 0 or new_coord[1] >= len(self.table):
                continue
            if self.is_next_step(self.table[new_coord[0]][new_coord[1]]):
                self.score += self.table[new_coord[0]][new_coord[1]].calculate_score()
        return self.score

def clear(nodes):
    for i in range(len(nodes)):
        for c in range(len(nodes[i])):
            nodes[i][c].score = -1
            nodes[i][c].visited = False
     

with open("input.txt") as f:
    content = f.readlines()
    content = [x.strip() for x in content]
    nodes = [[Node(int(c)) for c in line] for line in content]
    for i in range(len(nodes)):
        for c in range(len(nodes[i])):
            nodes[i][c].set_table_and_position(nodes, (i, c))

    res = 0
    for i in range(len(nodes)):
        for c in range(len(nodes[i])):
            if nodes[i][c].get_value() == 0:
                nodes[i][c].calculate_score()
                clear(nodes)
                res += score
                score = 0

    print(res)
