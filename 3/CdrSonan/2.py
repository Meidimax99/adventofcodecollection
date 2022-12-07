def getFirstCommonItem(nestedList):
    currentItems = []
    for i in nestedList:
        currentItems.append(i.pop(0))
    while True:
        for i in range(len(nestedList) - 1):
            if currentItems[i] != currentItems[i + 1]:
                minIndex = currentItems.index(min(currentItems))
                currentItems[minIndex] = nestedList[minIndex].pop(0)
                break
        else:
            return currentItems[0]

result = 0
setSize = 3
currentSet = []
with open("input", "r") as input:
    for i, line in enumerate(input):
        currentSet.append(sorted(line.rstrip("\n")))
        if i % 3 == 2:
            item = getFirstCommonItem(currentSet)
            currentSet = []
            prio = ord(item)
            if prio > 96:
                prio -= 96
            else:
                prio -= 38
            result += prio
print(result)
