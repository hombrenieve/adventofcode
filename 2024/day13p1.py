from collections import deque
from heapq import *


class DecisionTree:
    def __init__(self, game):
        self.game = game
        self.graph = {}
        # Build tree for 100 rounds
        # expanding with BFS
        rounds = 200
        visited = set()
        queue = deque([((0,0), 0)])
        while queue:
            node, round = queue.popleft()
            if node in visited:
                continue
            visited.add(node)
            if round == rounds:
                continue
            for neighbor in self.game.expand(node, self.graph):
                queue.append((neighbor, round + 1))
    
    def shortest_path(self):
        if self.game.prize not in self.graph:
            return -1
        source = (0, 0)
        distances = {node: 1e7 for node in self.graph}
        distances[source] = 0
        pq = [(0, source)]
        heapify(pq)
        visited = set()
        while pq:
            current_distance, current_node = heappop(pq)
            if current_node in visited:
                continue
            visited.add(current_node)

            for neighbor, weight in self.graph[current_node].items():
                distance = current_distance + weight
                if neighbor not in distances:
                    continue
                if distance < distances[neighbor]:
                    distances[neighbor] = distance
                    heappush(pq, (distance, neighbor))
        return distances[self.game.prize]

class Game:
    def __init__(self, a, b, prize):
        self.cost_a = 3
        self.cost_b = 1
        self.a = a
        self.b = b
        self.prize = prize
    
    def expand(self, position, adjacency):
        node_a = (position[0] + self.a[0], position[1] + self.a[1])
        node_b = (position[0] + self.b[0], position[1] + self.b[1])
        if position in adjacency:
            adjacency[position][node_a] = self.cost_a
            adjacency[position][node_b] = self.cost_b
        else:
            adjacency[position] = {node_a: self.cost_a, node_b: self.cost_b}
        return [node_a, node_b]


    
    def __repr__(self):
        return "A: " + str(self.a) + " B: " + str(self.b) + " P: " + str(self.prize)

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
        decisionTree = DecisionTree(game)
        res = decisionTree.shortest_path()
        print(nr, " ", res)
        nr += 1
        if res != -1:
            tokens += res
    print(tokens)
