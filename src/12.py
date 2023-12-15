from utils.api import get_input
from itertools import product
import re


input_str = get_input(12)

test_str = '''???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1'''

run_str = input_str

# WRITE YOUR SOLUTION HERE

def parse_input(string):
    springs = []
    guides = []
    for line in string.splitlines():
        spring = re.search(r'[?.#]+', line).group()
        springs.append(spring)

        guide = re.findall(r'[0-9]+', line)
        guide = [int(i) for i in guide]
        guides.append(guide)
    
    return springs, guides

def check_pattern(spring, guide):
    count = 0
    counts = []
    for index, c in enumerate(spring):
        if index == len(spring) - 1:
            if c == '#':
                count += 1
                counts.append(count)
            else:
                if count > 0:
                    counts.append(count)
        else:
            if c == '#':
                count += 1
            else:
                if count > 0:
                    counts.append(count)
                count = 0
    if counts == guide:
        return counts == guide

def generate_combinations(spring, guide, index=0, possible=[0]):
    if index == len(spring):
        if check_pattern(spring, guide) == True:
            possible[0] += 1
        return possible[0]

    if spring[index] == '?':
        spring = spring[:index] + '.' + spring[index + 1:]
        generate_combinations(spring, guide, index + 1, possible)

        spring = spring[:index] + '#' + spring[index + 1:]
        generate_combinations(spring, guide, index + 1, possible)
    else:
        generate_combinations(spring, guide, index + 1, possible)
    return possible[0]

springs, guides = parse_input(run_str)

result = 0
for i, line in enumerate(springs):
    result = generate_combinations(line, guides[i])

print(f'Part 1 Answer = {result}')

# Part 2:

def generate_combinations_2(spring, guide, index=0, memo = None):
    if memo is None:
        memo = {}

    if index == len(spring):
        if check_pattern(spring, guide):
            return 1
        return 0

    if (spring, index) in memo:
        return memo[(spring, index)]

    possible = 0
    if spring[index] == '?':
        spring_dot = spring[:index] + '.' + spring[index + 1:]
        possible += generate_combinations_2(spring_dot, guide, index + 1, memo)

        spring_hash = spring[:index] + '#' + spring[index + 1:]
        possible += generate_combinations_2(spring_hash, guide, index + 1, memo)
    else:
        possible += generate_combinations_2(spring, guide, index + 1, memo)

    memo[(spring, index)] = possible


    return possible




new_springs = []
for spring in springs:
   new_spring = '?'.join([spring] * 5) 
   new_springs.append(new_spring)

new_guides = []
for guide in guides:
    new_guide = guide * 5
    new_guides.append(new_guide)

result = 0

for i, line in enumerate(new_springs):
    result = generate_combinations_2(line, new_guides[i])
    total += result    
print(f'Part 2 Answer = {result}, or {total}')
