import sys
import os
from functools import reduce
import operator
from math import sqrt, ceil, floor

path = os.path.dirname(os.path.realpath(__file__)) + '/'
fname = path + 'input'
if len(sys.argv) > 1:
    fname = sys.argv[1]

file = open(fname)
lines = [line.rstrip() for line in file]

times = [int(strval) for strval in lines[0].split(':')[1].split(' ') if len(strval) > 0]
dists = [int(strval) for strval in lines[1].split(':')[1].split(' ') if len(strval) > 0]

races = zip(times, dists)

def getWaysToWin(time, dist):
    tMin = (time - sqrt(time**2 - 4*dist)) / 2
    tMax = (time + sqrt(time**2 - 4*dist)) / 2

    return floor(tMax) - ceil(tMin) + 1

# for each race, the number of charge times that result in a win
waysToWin = list()
for tup in races:
    waysToWin.append(getWaysToWin(*tup))
result1 = reduce(operator.mul, waysToWin, 1)
print('part1:', result1)

timeFull = int(''.join(str(i) for i in times))
distFull = int(''.join(str(i) for i in dists))
result2 = getWaysToWin(timeFull, distFull)
print('part2:', result2)
