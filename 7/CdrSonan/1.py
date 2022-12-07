smallThreshold = 100000
smallDirs = 0

def getSize(input) -> int:
    global smallDirs
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
    if size <= smallThreshold:
        smallDirs += size
    return size



with open("input", "r") as input:
    getSize(input)
print(smallDirs)