import sys
import os

path = os.path.dirname(os.path.realpath(__file__)) + '/'
fname = path + 'input'
if len(sys.argv) > 1:
    fname = sys.argv[1]

isDigit = lambda c: ('0' <= c <= '9')
isSymbol = lambda c: (c != '.') and not isDigit(c)

def getSymbolPositions(lines):
    symPos = list()
    for y, line in enumerate(lines):
        for x, char in enumerate(line):
            if isSymbol(char):
                symPos.append((x, y))

    return symPos

def getNumX(line, x):
    if not isDigit(line[x]):
        return None, None
    
    while isDigit(line[x-1]):
        x -= 1

    return x

def getNumber(line, x):
    x2 = x
    while x2 < len(line) and isDigit(line[x2]):
        x2 += 1

    return int(line[x:x2])

def getAdjacentNumbers(lines, x, y):
    numPos = list()
    for y1 in [y-1, y, y+1]:
        if not (0 <= y1 < len(lines)):
            continue
        for x1 in [x-1, x, x+1]:
            if not (0 <= x1 < len(lines[y1])):
                continue
            if isDigit(lines[y1][x1]):
                numX = getNumX(lines[y1], x1)
                if not (numX, y1) in numPos:
                    numPos.append((numX, y1))

    return numPos

sum = 0
gearSum = 0

file = open(fname)
lines = [line.rstrip() for line in file.readlines()]

symPos = getSymbolPositions(lines)

numPos = list()
for (x, y) in symPos:
    nums = getAdjacentNumbers(lines, x, y)
    numPos.extend(nums)

    # part 2: is a gear
    if lines[y][x] == '*' and len(nums) == 2:
        ratio = 1
        for (nx, ny) in nums:
            ratio *= getNumber(lines[ny], nx)
        
        gearSum += ratio

    # DEBUG
    # y0 = y-1 if y-1 >= 0 else 0
    # y1 = y+1 if y+1 < len(lines) else len(lines)-1
    # x0 = x-5 if x-5 >= 0 else 0
    # x1 = x+6 if x+6 < len(lines[y]) else len(lines[y])
    # print(*(getNumber(lines[y], x) for (x, y) in nums))
    # for y in range(y0, y1+1):
    #     print(lines[y][x0:x1])
    # print()
    # input()

for (x, y) in numPos:
    sum += getNumber(lines[y], x)

print('part1', sum)
print('part2', gearSum)
