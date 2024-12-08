    

antennas = {}
antinodes = set()
bounds=None

def in_bounds(coord):
    return coord[0] >= 0 and coord[0] < bounds[0] and coord[1] >= 0 and coord[1] < bounds[1]

def mark_antinnodes(id, nodes):
    for i in range(len(nodes)):
        for o in range(len(nodes)):
            if o == i: continue
            vector = (nodes[o][0]-nodes[i][0], nodes[o][1]-nodes[i][1])
            new_coord = (nodes[i][0] + 2*vector[0], nodes[i][1] + 2*vector[1])
            if in_bounds(new_coord):
                antinodes.add(new_coord)

with open("input.txt") as f:
    content = f.readlines()
    content = [x.strip() for x in content]
    content = [[c for c in line] for line in content]
    bounds = (len(content[0]), len(content))
    for i in range(len(content)):
        for c in range(len(content[i])):
            if content[i][c] != '.':
                if content[i][c] not in antennas:
                    antennas[content[i][c]] = []
                antennas[content[i][c]].append((c, i))

for key,value in antennas.items():
    mark_antinnodes(key, value)

print(len(antinodes))
