result = 0
dictLUT ={
    "A": {"X": 3, "Y": 4, "Z": 8},
    "B": {"X": 1, "Y": 5, "Z": 9},
    "C": {"X": 2, "Y": 6, "Z": 7}
}
with open("input", "r") as input:
    for line in input:
        result += dictLUT[line[0]][line[2]]
print(result)
