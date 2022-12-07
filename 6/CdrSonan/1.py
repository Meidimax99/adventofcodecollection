from copy import copy
bufferSize = 3
#bufferSize = 13 <- choose this buffer size for part two of the assignment
buffer = []
bufferIndex = 0
result = copy(bufferSize)
duplicateBlock = 0
with open("input", "r") as input:
    for i in range(bufferSize):
        buffer.append(input.read(1))
    while True:
        currentItem = input.read(1)
        result += 1
        duplicateBlock = max(0, duplicateBlock - 1)
        for i in range(bufferSize):
            effectiveIndex = (bufferIndex + i) % bufferSize
            if currentItem == buffer[effectiveIndex]:
                duplicateBlock = max(i + 1, duplicateBlock)
        if not duplicateBlock:
            break
        buffer[bufferIndex] = currentItem
        bufferIndex = (bufferIndex + 1) % bufferSize
print(result)
