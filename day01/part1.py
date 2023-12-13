import sys

fname = sys.argv[1]

with open(fname, "r") as file:
    sum = 0
    for line in file:
        first = None
        last = None
        for c in line:
            if '0' <= c <= '9':
                num = int(c)
                if first is None:
                    first = num
                last = num
        sum += (first * 10) + last

print(sum)
