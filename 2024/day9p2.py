
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

def search_free_space(records, size, end):
    for i in range(0, end+1):
        if records[i].is_free() and records[i].size >= size:
            return i
    return -1

def defrag(records):
    readi = len(records) - 1
    while readi > 0:
        if records[readi].used:
            writei = search_free_space(records, records[readi].size, readi)
            if writei != -1:
                records[writei].id = records[readi].id
                records[writei].used = True
                if records[writei].size > records[readi].size:
                    new_rec = DiskRecord(-1, records[writei].size-records[readi].size, False)
                    records[writei].size -= new_rec.size
                    records.insert(writei+1, new_rec)
                    readi += 1
                records[readi].id = -1
                records[readi].used = False
        readi -= 1
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
        for p in range(records[i].size):
            if records[i].used: sum += pos * records[i].id
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

    