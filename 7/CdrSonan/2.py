requiredSize = 70000000
def getSize(input) -> int:
    global smallestSize
    size = 0
    line = input.readline()
    while line != "$ cd ..\n" and line != "":
        if line.startswith("$ cd "):
            size += getSize(input)
        elif line.startswith("dir ") or line == "$ ls\n":
            pass
        else:
            size += int(line.split(" ")[0])
        line = input.readline()
    if size >= requiredSize:
        smallestSize = min(smallestSize, size)
    return size
with open("input", "r") as input:
    requiredSize = getSize(input) - 70000000 + 30000000
smallestSize = 70000000
with open("input", "r") as input:
    getSize(input)
print(smallestSize)