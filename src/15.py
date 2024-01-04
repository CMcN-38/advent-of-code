from utils.api import get_input
import re

input_str = get_input(15)

test_str = '''rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7'''

run_str = input_str

# WRITE YOUR SOLUTION HERE

def parse_string(string):
    parsed = string.split(",")

    return parsed

def hash(string):
    current_val = 0
    for char in string:
        current_val += ord(char)
        current_val = current_val * 17
        current_val = current_val % 256

    return current_val

parsed = parse_string(run_str)

total = 0
for str in parsed:
    total += hash(str)

print(f'Part 1 Answer: {total}')

# Part 2:

def parse_string(string):
    parsed = []
    for str in string.split(","):
        new_str = [re.search(r'^[a-z]+', str).group(), re.search(r'-|=', str).group()]
        if new_str[1] == "=":
            new_str.append(re.search(r'[0-9]+$', str).group())
        parsed.append(new_str)

    return parsed

def hash(string):
    current_val = 0
    for char in string:
        current_val += ord(char)
        current_val = current_val * 17
        current_val = current_val % 256

    return current_val

def hash_map(string):
    boxes = {}
    for code in string:
        box = hash(code[0])
        if box not in boxes:
            boxes[box] = {}
        if code[1] == "=":
            boxes[box][code[0]] = code[2]
        if code[1] == "-":
            if code[0] in boxes[box]:
                boxes[box].pop(code[0])

    return boxes

def calc_box(box_dict, box_number):
    total = 0
    for i, j in enumerate(box_dict):
        value = (box_number + 1) * (i + 1) * int(box_dict[j])
        total += value
    
    return total

parsed = parse_string(run_str)
hashed = hash_map(parsed)

result = 0
for box in hashed:
    result += calc_box(hashed[box], box)

print(f'Part 2 Answer: {result}')
