result = 0
with open("input", "r") as input:
    for line in input:
        half = int(len(line) / 2)
        left = sorted(line[:half])
        right = sorted(line[half:])
        currentLeft = left.pop(0)
        currentRight = right.pop(0)
        while True:
            if currentLeft < currentRight:
                currentLeft = left.pop(0)
            elif currentLeft > currentRight:
                currentRight = right.pop(0)
            else:
                break
        prio = ord(currentLeft)
        if prio > 96:
            prio -= 96
        else:
            prio -= 38
        result += prio
print(result)