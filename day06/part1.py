import sys
import os
from functools import reduce
import operator

path = os.path.dirname(os.path.realpath(__file__)) + '/'
fname = path + 'input'
if len(sys.argv) > 1:
    fname = sys.argv[1]

file = open(fname)
lines = [line.rstrip() for line in file]

times = [int(strval) for strval in lines[0].split(':')[1].split(' ') if len(strval) > 0]
dists = [int(strval) for strval in lines[1].split(':')[1].split(' ') if len(strval) > 0]

races = zip(times, dists)

# for each race, the number of charge times that result in a win
waysToWin = list()
for (time, dist) in races:
    numGood = 0
    for tCharge in range(time):
        tRace = time - tCharge
        speed = tCharge

        if speed * tRace > dist:
            numGood += 1

        elif numGood > 0:
            break

    waysToWin.append(numGood)

result = reduce(operator.mul, waysToWin, 1)
print(result)
