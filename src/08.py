from utils.api import get_input
import math
import re

real_str = get_input(8)

test_str = '''RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)'''

test_str_2 = '''LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)'''

test_str_3 = '''LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)'''

input_str = real_str
# WRITE YOUR SOLUTION HERE

def parse_input(string):
    maps = []
    for index, i in enumerate(string.splitlines()):
        if index == 0:
            directions = i
        elif i == '':
            continue
        else:
            #maps.append(re.findall(r'[A-Z]{3}', i))
            maps.append(re.findall(r'[0-9A-Z]{3}', i))

    return maps, directions

def follow_dir(maps, dir, loc):
    for sublist in maps:
        if sublist[0] ==  loc:
            if dir == 'L':
                return sublist[1]
            else:
                return sublist[2]
        else:
            continue

maps, directions = parse_input(input_str)

loc = "AAA"
steps = 0
while loc != "ZZZ":
    for c in directions:
        loc = follow_dir(maps, c, loc)
        steps += 1

print(f'Part 1 Answer: {steps}')

# PART 2:

input_str = real_str

def get_starts(list):
    starts = []
    for sublist in list:
        if sublist[0][2] == "A":
            starts.append(sublist[0])
    return starts

def check_starts(list):
    count = 0
    for start in list:
        if start[2] == "Z":
            count += 1
    if count == len(list):
        return True
    else:
        return False

maps, directions = parse_input(input_str)
starts = get_starts(maps)


all_steps = []
for loc in starts:
    steps = 0
    while loc[2] != "Z":
        for c in directions:
            loc = follow_dir(maps, c, loc)
            steps += 1
    all_steps.append(steps)

result = math.lcm(*all_steps)

print(f'Part 2 Answer: {result}')



