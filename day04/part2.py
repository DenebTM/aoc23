import sys
import os

path = os.path.dirname(os.path.realpath(__file__)) + '/'
fname = path + 'input'
if len(sys.argv) > 1:
    fname = sys.argv[1]

file = open(fname)
lines = file.readlines()
cards = list()

for line in lines:
    _, card = line.rstrip().split(': ')
    winning, have = card.split(' | ')
    winning = [int(num) for num in winning.split(' ') if len(num) > 0]
    have = [int(num) for num in have.split(' ') if len(num) > 0]

    # we have one instance of each card initially
    cards.append((winning, have, 1))

for i, (winning, have, instances) in enumerate(cards):
    numMatches = len([num for num in have if num in winning])

    # we win one instance each of the next `numMatches` cards after the current one
    # n instances of current card -> win n instances of each of those cards
    for i2 in range(i+1, i+1+numMatches):
        win2, have2, inst2 = cards[i2]
        cards[i2] = (win2, have2, inst2 + instances)

totalCards = sum(instances for (_, _, instances) in cards)
print(totalCards)
