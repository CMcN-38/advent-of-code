from utils.api import get_input
from collections import Counter
import re

real_str = get_input(7)

test_str = '''32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483'''

input_str = real_str

# WRITE YOUR SOLUTION HERE

def parse_hand(string):
    hand = re.search(r'[0-9A-Z]{5}', string).group()
    bet = int(re.search(r'[0-9]+$', string).group())
    return hand, bet

def check_hand(string):
    count = Counter(string)
    values = sorted(count.values(), reverse = True)

    if 5 in values:
        return 7
    elif 4 in values:
        return 6
    elif 3 in values and 2 in values:
        return 5
    elif 3 in values:
        return 4
    elif values.count(2) == 2:
        return 3
    elif 2 in values:
        return 2
    else:
        return 1 

def get_rank(string):
    hands_rank = []
    for index, i in enumerate(string.splitlines()):
        hand, bet = parse_hand(i)
        hand_res = check_hand(hand)
        hands_rank.append([hand, bet, hand_res])
    
    #char_ranks = {'2': 12, '3': 11, '4': 10, '5': 9, '6': 8, '7': 7, '8': 6, '9': 5, 'T': 4, 'J': 3, 'Q': 2, 'K': 1, 'A': 0}
    char_ranks = {'2': 0, '3': 1, '4': 2, '5': 3, '6': 4, '7': 5, '8': 6, '9': 7, 'T': 8, 'J': 9, 'Q': 10, 'K': 11, 'A': 12}
   
    hands_rank.sort(key=lambda x: (x[2], [char_ranks[c] for c in x[0]]))

    rank = 0
    prev_value = None
    prev_rank = 0

    for i, sublist in enumerate(hands_rank):
        sublist.append(i+1) 
    return hands_rank 

def calc_winnings(list):
    total = 0
    for sublist in list:
        sublist.append(sublist[1] * sublist[3])
        total += sublist[4]
    return total
    
result = get_rank(input_str)
total_win = calc_winnings(result)

print(f'Part 1 Answer: {total_win}')
