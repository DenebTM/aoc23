import sys
import os

path = os.path.dirname(os.path.realpath(__file__)) + '/'
fname = path + 'input'
if len(sys.argv) > 1:
    fname = sys.argv[1]

score = 0

file = open(fname)

for card in file:
    _, card = card.rstrip().split(': ')
    winning, have = card.split(' | ')
    winning = [int(num) for num in winning.split(' ') if len(num) > 0]
    have = [int(num) for num in have.split(' ') if len(num) > 0]

    points = 0
    for num in have:
        if num in winning:
            if points == 0:
                points = 1
            else:
                points *= 2
    
    score += points

print(score)
