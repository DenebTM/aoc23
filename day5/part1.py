import sys
import os

path = os.path.dirname(os.path.realpath(__file__)) + '/'
fname = path + 'input'
if len(sys.argv) > 1:
    fname = sys.argv[1]

file = open(fname)
lines = [line.rstrip() for line in file]

seeds = list()
maps = dict()

def get_dest(map, src):
    for src_0, (dest_0, length) in map.items():
        if src_0 <= src < (src_0 + length):
            return dest_0 + (src - src_0)

    return src

# note to self: do not attempt to store 200+ million elements in a dict
def update_map(map, line_iter):
    for line in line_iter:
        print(line)
        if len(line) == 0:
            return
        
        dest_0, src_0, length = [int(num) for num in line.split(' ') if len(num) > 0]
        map[src_0] = (dest_0, length)


line_iter = iter(lines)
for line in line_iter:
    print(line)
    if 'seeds' in line:
        seeds = [int(seed) for seed in line.split(': ')[1].split(' ')]

    elif 'map' in line:
        map_name = line.split(' ')[0]
        if not map_name in maps:
            maps[map_name] = dict()
        update_map(maps[map_name], line_iter)


lowest = None
for seed in seeds:
    dest = seed
    # python dicts preserve insert order :3
    for map_name in maps:
        map = maps[map_name]

        src = dest
        dest = get_dest(map, src)

    if lowest is None or dest < lowest:
        lowest = dest

print(lowest)
