import sys
import os
import tqdm
from concurrent.futures import ThreadPoolExecutor as Executor, as_completed

path = os.path.dirname(os.path.realpath(__file__)) + '/'
fname = path + 'input'
if len(sys.argv) > 1:
    fname = sys.argv[1]

file = open(fname)
lines = [line.rstrip() for line in file]

seeds = list()
maps = dict()
rev_maps = dict()

def get_dest(map, src):
    for src_0, (dest_0, length) in map.items():
        if src_0 <= src < (src_0 + length):
            return dest_0 + (src - src_0)

    return src

# each mapping appears to be injective; no overlapping ranges, but not every destination is mapped
def get_src(map, dest):
    for src_0, (dest_0, length) in map.items():
        if dest_0 <= dest < (dest_0 + length):
            return src_0 + (dest - dest_0)

    if get_dest(map, dest) == dest:
        return dest

    return None

# note to self: do not attempt to store 200+ million elements in a dict
def update_map(map, line_iter):
    for line in line_iter:
        if len(line) == 0:
            return
        
        dest_0, src_0, length = [int(num) for num in line.split(' ') if len(num) > 0]
        map[src_0] = (dest_0, length)

def seed_exists(seeds, seed):
    for (seed_0, count) in seeds:
        if seed_0 <= seed < (seed_0 + count):
            return True
    
    return False

line_iter = iter(lines)
for line in line_iter:
    if 'seeds' in line:
        seeds = [int(seed) for seed in line.split(': ')[1].split(' ')]
        seeds = list(zip(seeds[::2], seeds[1::2]))

    elif 'map' in line:
        map_name = line.split(' ')[0]
        if not map_name in maps:
            maps[map_name] = dict()
        update_map(maps[map_name], line_iter)


# lowest = None
# for (seed_0, count) in seeds:
#     print(seed_0)
#     for i in range(count):
#         dest = seed_0 + i
#         # python dicts preserve insert order :3
#         for map_name in maps:
#             map = maps[map_name]

#             src = dest
#             dest = get_dest(map, src)

#         if lowest is None or dest < lowest:
#             lowest = dest

#total = int(5e10)
start = int(0)
total = int(2e7)
lowest = None

## ALT 1
with tqdm.trange(start, total) as t:
    for location in t:
        src = location

        # python dicts preserve insert order :3
        for map_name in reversed(maps):
            map = maps[map_name]

            dest = src
            src = get_src(map, dest)
            if src is None:
                break

        if src is not None and seed_exists(seeds, src):
            lowest = location
            break


## ALT 2
# max_workers = 24
# part = int(total / max_workers)

# with tqdm.tqdm(total=total) as pbar:
#     def worker(ran):
#         for location in ran:
#             src = location

#             for map_name in reversed(maps):
#                 map = maps[map_name]

#                 dest = src
#                 src = get_src(map, dest)
#                 if src is None:
#                     break

#             pbar.update(1)

#             # abort worker thread if lower element already found
#             if lowest is not None and lowest < location:
#                 return None
            
#             # found lower element
#             if src is not None and seed_exists(seeds, src):
#                 return location

#     with Executor(max_workers=max_workers) as executor:
#         futures = [executor.submit(worker, range(base, base+part)) for base in range(0, total, part)]

#         for future in as_completed(futures):
#             pbar.total -= part
#             result = future.result()
#             if result is not None and (lowest is None or result < lowest):
#                 lowest = result
#                 print(f'{lowest}...')


print(lowest)
