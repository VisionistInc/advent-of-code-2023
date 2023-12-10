import sys

with open(sys.argv[1]) as f:
    data = f.read()

lines = data.split('\n')

def predictNext(vals):
    if vals.count(0) == len(vals):
        return 0
    else:
        new_vals = []
        for i in range(len(vals)-1):
            new_vals.append(vals[i+1] - vals[i])
        x = predictNext(new_vals)
        return x + vals[-1]

def readPast(vals):
    if vals.count(0) == len(vals):
        return 0
    else:
        new_vals = []
        for i in range(len(vals)-1):
            new_vals.append(vals[i+1] - vals[i])
        x = readPast(new_vals)
        return vals[0] - x

future = 0
past = 0
for line in lines:
    line = line.split(' ')
    for x in range(len(line)):
        line[x] = int(line[x])
    future += predictNext(line)
    past += readPast(line)

print("Part 1:", future)
print("Part 2:", past)
