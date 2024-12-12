
class Node:
    def __init__(self, value, location, table):
        self.value = value
        self.visited = False
        self.location = location
        self.table = table
    
    def get_perimeter_count(self):
        res = 0
        for move in [(-1,0), (1,0), (0,-1), (0,1)]:
            new_coord = (self.location[0] + move[0], self.location[1] + move[1])
            if new_coord[0] < 0 or new_coord[0] >= len(self.table[0]) or new_coord[1] < 0 or new_coord[1] >= len(self.table):
                res += 1
            elif self.table[new_coord[0]][new_coord[1]].value != self.value:
                res += 1
        return res
    
    def get_visitable_nodes(self):
        res = []
        for move in [(-1,0), (1,0), (0,-1), (0,1)]:
            new_coord = (self.location[0] + move[0], self.location[1] + move[1])
            if new_coord[0] < 0 or new_coord[0] >= len(self.table[0]) or new_coord[1] < 0 or new_coord[1] >= len(self.table):
                continue
            elif self.table[new_coord[0]][new_coord[1]].value == self.value and not self.table[new_coord[0]][new_coord[1]].visited:
                res.append(self.table[new_coord[0]][new_coord[1]])
        return res
    
    def visit_area(self):
        if self.visited:
            return 0
        perimeter = self.get_perimeter_count()
        area = 1
        self.visited = True
        q = self.get_visitable_nodes()
        while len(q) > 0:
            node = q.pop(0)
            if node.visited:
                continue
            node.visited = True
            perimeter += node.get_perimeter_count()
            area += 1
            q += node.get_visitable_nodes()
        return perimeter*area
    
    def __repr__(self):
        return str(self.value + " " + str(self.visited) + " " + str(self.get_perimeter_count()))

def calculate_perimeter_cost(table):
    res = 0
    for i in range(len(table)):
        for c in range(len(table[i])):
            node = table[i][c]
            res += node.visit_area()
    return res

with open('input.txt') as f:
    content = [i.strip() for i in f.readlines()]
    content = [[c for c in line] for line in content]
    table = []
    for i in range(len(content)):
        table.append([])
        for c in range(len(content[i])):
            table[i].append(Node(content[i][c], (i, c), table))
    print(calculate_perimeter_cost(table))