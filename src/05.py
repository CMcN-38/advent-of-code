from utils.api import get_input
import re

real_str = get_input(5)

# WRITE YOUR SOLUTION HERE

test_str = '''seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
'''
def get_map(string):
    map_name = string.split('\n')[0]
    map_name = re.search(r'^[a-z\\-]+', map_name).group()
    map_formatted = [map_name]

    trios = re.findall(r'[0-9][0-9 ]+', string)
    trios_ext = []
    for string in trios:
        numbers = string.split()
        trios_ext.append(numbers)
 
    destinations = []
    sources = []
    for sublist in trios_ext:
        des_start = int(sublist[0])
        des_end = int(sublist[0])+int(sublist[2])
        destinations.extend(range(des_start, des_end))
        sou_start = int(sublist[1])
        sou_end = int(sublist[1])+int(sublist[2])
        sources.extend(range(sou_start, sou_end)) 
    return map_name, destinations, sources

def follow_map(source, sour_list, dest_list):
    if source in sour_list:
        index = sour_list.index(source)
        return dest_list[index]
    else:
        return source
    


input_str = real_str

seeds = input_str.split('\n')[0]
seeds = [int(s) for s in seeds.split() if s.isdigit()]

raw_maps = re.findall(r'[a-z\- ]+:\n[0-9 \n]+', input_str)

map_1_name, map_1_dest, map_1_sour = get_map(raw_maps[0])
map_2_name, map_2_dest, map_2_sour = get_map(raw_maps[1])
map_3_name, map_3_dest, map_3_sour = get_map(raw_maps[2])
map_4_name, map_4_dest, map_4_sour = get_map(raw_maps[3])
map_5_name, map_5_dest, map_5_sour = get_map(raw_maps[4])
map_6_name, map_6_dest, map_6_sour = get_map(raw_maps[5])
map_7_name, map_7_dest, map_7_sour = get_map(raw_maps[6])

all_soils = []
all_ferts = []
all_waters = []
all_lights = []
all_temps = []
all_humids = []
all_locs = []

for s in seeds:
    soil = follow_map(s, map_1_sour, map_1_dest)
    all_soils.append(soil)

    fert = follow_map(soil, map_2_sour, map_2_dest)
    all_ferts.append(fert)

    water = follow_map(fert, map_3_sour, map_3_dest)
    all_waters.append(water)

    light = follow_map(water, map_4_sour, map_4_dest)
    all_lights.append(light)

    temp = follow_map(light, map_5_sour, map_5_dest)
    all_temps.append(temp)

    humid = follow_map(temp, map_6_sour, map_6_dest)
    all_humids.append(humid)
    
    loc = follow_map(humid, map_7_sour, map_7_dest)
    all_locs.append(loc)

print(f'Part 1: {min(all_locs)}')
