from utils.api import get_input
import re

input_str = get_input(2)

# WRITE YOUR SOLUTION HERE
total = 0

#12R, 13G, 14B

def is_possible(string):

    reds = re.findall(r'[0-9]+\sred', string)
    blues = re.findall(r'[0-9]+\sblue', string)
    greens = re.findall(r'[0-9]+\sgreen', string)
 
    reds = [item.replace(' red', '') for item in reds]
    blues = [item.replace(' blue', '') for item in blues]
    greens = [item.replace(' green', '') for item in greens]

    if any(int(item) > 12 for item in reds):
        return False
    if any(int(item) > 13 for item in greens):
        return False
    if any(int(item) > 14 for item in blues):
        return False
    else:
        return True

id = 0
for i in input_str.splitlines():
    id += 1
    if is_possible(i) == True:
        total += id
    else:
        continue

print(f'The total for Part 1 is: {total}')

# Part 2

total = 0

#12R, 13G, 14B

def min_needed(string):
    RGB = [0,0,0]

    reds = re.findall(r'[0-9]+\sred', string)
    blues = re.findall(r'[0-9]+\sblue', string)
    greens = re.findall(r'[0-9]+\sgreen', string)
 
    reds = [item.replace(' red', '') for item in reds]
    blues = [item.replace(' blue', '') for item in blues]
    greens = [item.replace(' green', '') for item in greens]

    RGB[0] = max(int(item) for item in reds)
    RGB[1] = max(int(item) for item in greens)
    RGB[2] = max(int(item) for item in blues)

    return RGB

id = 0
for i in input_str.splitlines():
    RGB = min_needed(i)
    power = RGB[0] * RGB[1] * RGB[2]
    total += power   

print(f'The total for Part 2 is: {total}')


