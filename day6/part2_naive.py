import sys
import os

path = os.path.dirname(os.path.realpath(__file__)) + '/'
fname = path + 'input'
if len(sys.argv) > 1:
    fname = sys.argv[1]

file = open(fname)
lines = [line.rstrip() for line in file]

time = int(''.join(strval for strval in lines[0].split(':')[1].split(' ') if len(strval) > 0))
dist = int(''.join(strval for strval in lines[1].split(':')[1].split(' ') if len(strval) > 0))

print(time, dist)

result = 0
for tCharge in range(time):
    tRace = time - tCharge
    speed = tCharge
    
    if speed * tRace > dist:
        result += 1
    
    elif result > 0:
        break

print(result)
