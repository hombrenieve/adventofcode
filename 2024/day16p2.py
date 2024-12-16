from heapq import heapify, heappop, heappush
from collections import deque

class Maze:
    def __init__(self, content):
        self.content = content
        self.nodes = []
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
                    for dir in ['N', 'E', 'S', 'W']:
                        node = ((i, j), dir)
                        if node not in self.graph:
                            self.graph[node] = {}
        for i in range(len(self.maze.content)):
            for j in range(len(self.maze.content[i])):
                if self.maze.content[i][j] in ['.', 'S', 'E']:
                    # rotations
                    for dir in ['N', 'E', 'S', 'W']:
                        node = ((i, j), dir)
                        if dir == 'S':
                            node1 = ((i, j), 'E')
                            self.graph[node][node1] = 1000
                            node2 = ((i, j), 'W')
                            self.graph[node][node2] = 1000
                        if dir == 'E':
                            node1 = ((i, j), 'N')
                            self.graph[node][node1] = 1000
                            node2 = ((i, j), 'S')
                            self.graph[node][node2] = 1000
                        if dir == 'W':
                            node1 = ((i, j), 'N')
                            self.graph[node][node1] = 1000
                            node2 = ((i, j), 'S')
                            self.graph[node][node2] = 1000
                        if dir == 'N':
                            node1 = ((i, j), 'W')
                            self.graph[node][node1] = 1000
                            node2 = ((i, j), 'E')
                            self.graph[node][node2] = 1000
                    # movements
                    for dir in ['N', 'E', 'S', 'W']:
                        node = ((i, j), dir)
                        if dir == 'N':
                            if ((i-1, j), 'N') in self.graph:
                                node1 = ((i-1, j), 'N')
                                self.graph[node][node1] = 1
                        if dir == 'E':
                            if ((i, j+1), 'E') in self.graph:
                                node1 = ((i, j+1), 'E')
                                self.graph[node][node1] = 1
                        if dir == 'S':
                            if ((i+1, j), 'S') in self.graph:
                                node1 = ((i+1, j), 'S')
                                self.graph[node][node1] = 1
                        if dir == 'W':
                            if ((i, j-1), 'W') in self.graph:
                                node1 = ((i, j-1), 'W')
                                self.graph[node][node1] = 1

    def shortest_distance(self):
        source = (self.maze.start, 'E')
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
                if distance < distances[neighbor]:
                    distances[neighbor] = distance
                    heappush(pq, (distance, neighbor))
        results = [distances[(self.maze.end, x)] for x in ['N', 'E', 'S', 'W']]
        minimum = min(results)
        i = results.index(minimum)
        target = (self.maze.end, ['N', 'E', 'S', 'W'][i])
        predecessors = {node: None for node in self.graph}
        for node, distance in distances.items():
            for neighbor, weight in self.graph[node].items():
                if distances[neighbor] == distance + weight:
                    if predecessors[neighbor] is None:
                        predecessors[neighbor] = [node]
                    else:
                        predecessors[neighbor].append(node)
        return distances, predecessors, target
    
    def shortest_paths(self):
        distances, predecessors, target = self.shortest_distance()
        path = set()
        node = [target]
        while len(node) > 0:
            newn = []
            for n in node:
                path.add(n[0])
                if predecessors[n] is None:
                    continue
                newn += predecessors[n]
            node = newn
        return len(path)

with open("input.txt") as f:
    content = f.readlines()
    content = [[c for c in line.strip()] for line in content]
    maze = Maze(content)
    graph = Graph(maze)
    print(graph.shortest_paths())
    