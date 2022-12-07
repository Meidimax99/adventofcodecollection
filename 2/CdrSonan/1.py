result = 0
dictLUT ={
    "A": {"X": 4, "Y": 8, "Z": 3},
    "B": {"X": 1, "Y": 5, "Z": 9},
    "C": {"X": 7, "Y": 2, "Z": 6}
}
with open("input", "r") as input:
    for line in input:
        result += dictLUT[line[0]][line[2]]
print(result)
