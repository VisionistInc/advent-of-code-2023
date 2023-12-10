import sys
import math

with open(sys.argv[1]) as f:
    data = f.read()

lines = data.split('\n')

m = {}

path = lines[0]

all_a = []

for line in lines[2:]:
    node = line[0:3]
    left = line[7:10]
    right = line[12:15]
    lr = {}
    lr['L'] = left
    lr['R'] = right
    m[node] = lr
    if node[-1] == 'A':
        all_a.append(node)

pos = 'AAA'
steps = 0
while pos != 'ZZZ':
    pos = m[pos][path[steps % len(path)]]
    steps += 1

print("Part 1:", steps)

# get how many steps it takes for each starting point
all = []
for pos in all_a:
    steps = 0
    while pos[-1] != 'Z':
        pos = m[pos][path[steps % len(path)]]
        steps += 1
    all.append(steps)

# get the gcd across all the numbers
gcd = max(all)
for i in range(len(all)-1):
    _gcd = math.gcd(all[i], all[i+1])
    if _gcd < gcd:
        gcd = _gcd

# get all the unique factors so we can make a least common multiple
factors = []
for n in all:
    factor = n//gcd
    if not factor in factors:
        factors.append(factor)

lcm = gcd
for f in factors:
    lcm *= f

print("Part 2:", lcm)
