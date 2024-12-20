from heapq import heapify, heappop, heappush

class Maze:
    def __init__(self, content):
        self.content = content
        for i in range(len(self.content)):
            for j in range(len(self.content[i])):
                if self.content[i][j] == "S":
                    self.start = (i, j)
                if self.content[i][j] == "E":
                    self.end = (i, j)

class Graph:
    def __init__(self, maze):
        self.maze = maze
        self.graph = {}
        # Create adjacency list
        for i in range(len(self.maze.content)):
            for j in range(len(self.maze.content[i])):
                if self.maze.content[i][j] in ['.', 'S', 'E']:
                        self.graph[(i, j)] = []
                        for m in [(0,1), (0,-1), (1,0), (-1,0)]:
                            if self.maze.content[i+m[0]][j+m[1]] in ['.', 'S', 'E']:
                                self.graph[(i, j)].append((i+m[0], j+m[1]))
    
    def add_node(self, node):
        self.graph[node] = []
        for m in [(0,1), (0,-1), (1,0), (-1,0)]:
            if self.maze.content[node[0]+m[0]][node[1]+m[1]] in ['.', 'S', 'E']:
                self.graph[node].append((node[0]+m[0], node[1]+m[1]))
            nodeN = (node[0]+m[0], node[1]+m[1])
            if nodeN in self.graph:
                self.graph[nodeN].append(node)

    def remove_node(self, node):
        for neighbor in self.graph[node]:
            self.graph[neighbor].remove(node)
        del self.graph[node]
        for m in [(0,1), (0,-1), (1,0), (-1,0)]:
            nodeN = (node[0]+m[0], node[1]+m[1])
            if nodeN in self.graph:
                if node in self.graph[nodeN]:
                    self.graph[nodeN].remove(node)

    def shortest_distance(self):
        source = self.maze.start
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
            for neighbor in self.graph[current_node]:
                distance = current_distance + 1
                if distance < distances[neighbor]:
                    distances[neighbor] = distance
                    heappush(pq, (distance, neighbor))
        return distances[self.maze.end]

class Game:
    def __init__(self, content):
        self.maze = Maze(content)
        self.graph = Graph(self.maze)
    
    def calculate_cheat_distance(self, saved_distance_threshold):
        times = 0
        base_distance = self.graph.shortest_distance()
        total = (len(self.maze.content)-1) * (len(self.maze.content[0])-1)
        for i in range(1, len(self.maze.content)-1):
            for j in range(1, len(self.maze.content[i])-1):
                total -= 1
                if self.maze.content[i][j] == '#':
                    self.graph.add_node((i, j))
                    if self.graph.shortest_distance() <= base_distance - saved_distance_threshold:
                        times += 1
                    self.graph.remove_node((i, j))
                print(total, times)

        return times

with open("input.txt") as f:
    content = f.readlines()
    content = [[c for c in line.strip()] for line in content]
    game = Game(content)
    print(game.calculate_cheat_distance(100))
    