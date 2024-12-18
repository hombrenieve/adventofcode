from heapq import heapify, heappop, heappush


class Graph:

    def __init__(self, grid_size, blocked):
        self.grid_size = grid_size
        self.blocked = blocked
        self.graph = {}  # adjacency list
        for y in range(grid_size):
            for x in range(grid_size):
                if (x, y) in self.blocked:
                    continue
                self.graph[(x, y)] = []
                if x > 0 and (x - 1, y) not in self.blocked:
                    self.graph[(x, y)].append((x - 1, y))
                if x < grid_size - 1 and (x + 1, y) not in self.blocked:
                    self.graph[(x, y)].append((x + 1, y))
                if y > 0 and (x, y - 1) not in self.blocked:
                    self.graph[(x, y)].append((x, y - 1))
                if y < grid_size - 1 and (x, y + 1) not in self.blocked:
                    self.graph[(x, y)].append((x, y + 1))
    
    def shortest_distance(self):
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

            for neighbor in self.graph[current_node]:
                distance = current_distance + 1
                if distance < distances[neighbor]:
                    distances[neighbor] = distance
                    heappush(pq, (distance, neighbor))
        return distances[(self.grid_size - 1, self.grid_size - 1)]


with open('input.txt') as f:
    content = [i.strip() for i in f.readlines()]
    content = [(int(x), int(y)) for x, y in [line.split(',') for line in content]]
    print(Graph(71, content[0:1024]).shortest_distance())