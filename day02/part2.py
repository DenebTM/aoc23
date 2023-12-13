import sys

fname = sys.argv[1]

with open(fname, "r") as games:
    sum = 0
    for game in games:
        id, subsets = game.split(": ")
        id = int(id.replace("Game ", ""))

        min = {
            "red": 0,
            "green": 0,
            "blue": 0
        }
        for subset in subsets.split("; "):
            counts = subset.split(", ")
            
            for count, color in [c.split(" ") for c in counts]:
                count = int(count)
                color = color.rstrip()
                if count > min[color]:
                    min[color] = count

        power = min["red"] * min["green"] * min["blue"]
        sum += power

print(sum)
