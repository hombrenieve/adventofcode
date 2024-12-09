
class DiskRecord:
    def __init__(self, id, size, used):
        self.id = id
        self.size = size
        self.used = used
    
    def is_free(self):
        return self.used == False

def load_records(sequence):
    records = []
    id = 0
    used = True
    for rec in sequence:
        if used:
            records.append(DiskRecord(id, rec, True))
            id += 1
        elif rec != 0:
            records.append(DiskRecord(-1, rec, False))
        used = not used
    return records

def defrag(records):
    writei = 0
    readi = len(records) - 1
    # look for empty space
    while(records[writei].used and writei < readi):
        writei += 1
    # look for used space to move
    while(not records[readi].used and readi > writei):
        readi -= 1
    while writei < readi:
        if records[writei].size >= records[readi].size:
            records[writei].id = records[readi].id
            records[writei].used = True
            if records[writei].size > records[readi].size:
                new_rec = DiskRecord(-1, records[writei].size-records[readi].size, False)
                records[writei].size -= new_rec.size
                records.insert(writei+1, new_rec)
                readi += 1
            records[readi].id = -1
            records[readi].used = False
            while records[writei].used and writei < readi:
                writei += 1
            while not records[readi].used and readi > writei:
                readi -= 1
        else:
            records[writei].id = records[readi].id
            records[writei].used = True
            records[readi].size -= records[writei].size
            while records[writei].used and writei < readi:
                writei += 1
    # compact
    i = 0
    while i < len(records) - 1:
        if records[i].id == records[i+1].id:
            records[i].size += records[i+1].size
            del records[i+1]
        else:
            i += 1

def checksum(records):
    sum = 0
    pos = 0
    for i in range(len(records)):
        if records[i].used:
            for p in range(records[i].size):
                sum += pos * records[i].id
                pos += 1
    return sum

def print_records(records):
    for rec in records:
        print("id:", rec.id, "size:", rec.size, "used:", rec.used)

with open("input.txt") as f:
    content = f.readlines()
    content = [x.strip() for x in content]
    content = [[c for c in line] for line in content]
    content = [int(x) for x in content[0]]
    records = load_records(content)
    defrag(records)
    print(checksum(records))

    