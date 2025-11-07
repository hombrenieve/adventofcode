number = 361527
ring = 0
gen = 1
i = 1
while i <= number:
    print("Ring: ", ring, " Gen: ", gen, " From: ", i, " to: ", gen*gen)
    ring += 1
    i = gen*gen+1
    gen += 2

orig = (gen-4)*(gen-4)+1
side = gen-3
rest = (number-orig)
first = orig+side//2
centers = [first-1, first+side-1, first+2*side-1, first+3*side-1]
difs = [abs(number-y) for y in centers]
print(centers)
print(difs)
m = min(difs)
print(m)

print("total: ", ring-1+m)