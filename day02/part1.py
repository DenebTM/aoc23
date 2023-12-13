import sys

fname = sys.argv[1]

max = {
    "red": 12,
    "green": 13,
    "blue": 14
}

with open(fname, "r") as games:
    sum = 0
    for game in games:
        id, subsets = game.split(": ")
        id = int(id.replace("Game ", ""))

        validGame = True
        for subset in subsets.split("; "):
            counts = subset.split(", ")
            
            for count, color in [c.split(" ") for c in counts]:
                count = int(count)
                if count > max[color.rstrip()]:
                    validGame = False
                    break

            if not validGame:
                break

        if validGame:
            sum += id

print(sum)
