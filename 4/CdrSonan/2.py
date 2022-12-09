result = 0
with open("input", "r") as input:
    for line in input:
        leftLine, rightLine = line.split(",")
        leftLine = leftLine.split("-")
        rightLine = rightLine.split("-")
        leftLine[0] = int(leftLine[0])
        leftLine[1] = int(leftLine[1])
        rightLine[0] = int(rightLine[0])
        rightLine[1] = int(rightLine[1])
        if leftLine[1] < rightLine[0]:
            continue
        elif leftLine[0] > rightLine[1]:
            continue
        result += 1
print(result)
