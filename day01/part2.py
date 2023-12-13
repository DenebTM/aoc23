import sys

fname = sys.argv[1]

strs = { "one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6, "seven": 7, "eight": 8, "nine": 9 }

with open(fname, "r") as file:
    sum = 0
    for line in file:
        first = None
        last = None

        first_ind = len(line)
        last_ind = -1
        for s in strs:
            ind = line.find(s)
            if ind != -1 and ind < first_ind:
                first_ind = ind
                first = strs[s]

            lind = line.rfind(s)
            if lind > last_ind:
                last_ind = lind
                last = strs[s]
                
        for i, c in enumerate(line):
            if '1' <= c <= '9':
                num = int(c)
                if first is None or i < first_ind:
                    first_ind = i
                    first = num
                if i > last_ind:
                    last_ind = i
                    last = num
        sum += (first * 10) + last

        # print(line.strip())
        # print(first, last)
        # input()

print(sum)
