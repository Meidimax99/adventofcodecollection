from collections import deque
numBins = 9
bins = []
result = []
for i in range(numBins):
    bins.append(deque([]))
with open("input", "r") as input:
    line = input.readline()
    while line != " 1   2   3   4   5   6   7   8   9 \n":
        for i in range(numBins):
            item = line[4 * i + 1]
            if item != " ":
                bins[i].appendleft(item)
        line = input.readline()
    line = input.readline()
    line = input.readline()
    while line:
        amount = int(line[5:-13])
        source = int(line[-7]) - 1
        target = int(line[-2]) - 1
        for i in range(amount):
            bins[target].append(bins[source].pop())
        line = input.readline()
for i in bins:
    result.append(i.pop())
result = "".join(result)
print(result)
