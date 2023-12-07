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

def check_hand(string, part):
    if part == 1:
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
    if part == 2:
        wildcards = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A']
        hands = []
    
        for wildcard in wildcards:
            modified_hand = [card if card != 'J' else wildcard for card in string]
            count = Counter(modified_hand)
            values = sorted(count.values(), reverse = True)

            if 5 in values:
                hands.append(7)
            elif 4 in values:
                hands.append(6)
            elif 3 in values and 2 in values:
                hands.append(5)
            elif 3 in values:
                hands.append(4)
            elif values.count(2) == 2:
                hands.append(3)
            elif 2 in values:
                hands.append(2)
            else:
                hands.append(1)
        return max(hands)


 
def get_rank(string, part):
    hands_rank = []
    for index, i in enumerate(string.splitlines()):
        hand, bet = parse_hand(i)
        hand_res = check_hand(hand, part)
        hands_rank.append([hand, bet, hand_res])
    
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
    
char_ranks = {'2': 0, '3': 1, '4': 2, '5': 3, '6': 4, '7': 5, '8': 6, '9': 7, 'T': 8, 'J': 9, 'Q': 10, 'K': 11, 'A': 12}

result = get_rank(input_str, 1)
total_win = calc_winnings(result)

print(f'Part 1 Answer: {total_win}')

## Part 2

char_ranks = {'2': 1, '3': 2, '4': 3, '5': 4, '6': 5, '7': 6, '8': 7, '9': 8, 'T': 9, 'J': 0, 'Q': 10, 'K': 11, 'A': 12}

result = get_rank(input_str, 2)
total_win = calc_winnings(result)

print(f'Part 2 Answer: {total_win}')


