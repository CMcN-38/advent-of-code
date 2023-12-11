from utils.api import get_input
from collections import deque
input_str = get_input(11)

test_str = '''...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....'''

run_str = input_str

# WRITE YOUR SOLUTION HERE
def parse_input(string):
    new_grid = []
    for row in string.splitlines():
        if all (x == '.' for x in row):
            new_row = 'X' * len(row)
            new_grid.append(new_row)
        else:
            new_grid.append(row)
    transposed_grid = [''.join(row) for row in zip(*new_grid)]

    new_transposed_grid = []
    for row in transposed_grid:
        if all (x in ('.', 'X') for x in row):
            new_row = 'X' * len(row)
            new_transposed_grid.append(new_row)
        else:
            new_transposed_grid.append(row)

    expanded_grid = [''.join(row) for row in zip(*new_transposed_grid)]

    return expanded_grid

def shortest_path(grid, mod):
    def get_neighbors(x, y):
        directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]
        neighbors = []
        for dx, dy in directions:
            nx, ny = x + dx, y + dy
            if (
                0 <= nx < len(grid)
                and 0 <= ny < len(grid[0])
            ):
                neighbors.append((nx, ny))
        return neighbors

    def bfs(start):
        queue = deque([(start, 0)])
        visited = {start}
        while queue:
            (x, y), distance = queue.popleft()
            for neighbor in get_neighbors(x, y):
                if neighbor not in visited:
                    queue.append((neighbor, distance + 1 if grid[neighbor[0]][neighbor[1]] != 'X' else distance + mod))
                    visited.add(neighbor)
                    if grid[neighbor[0]][neighbor[1]] == '#':
                        distances[start][neighbor] = distance + 1 if grid[neighbor[0]][neighbor[1]] != 'X' else distance + mod

    # Get positions of all '#' characters
    positions = [(i, j) for i in range(len(grid)) for j in range(len(grid[0])) if grid[i][j] == '#']

    # Initialize distances matrix
    distances = {pos: {other_pos: float('inf') for other_pos in positions} for pos in positions}

    # Calculate shortest paths
    for pos in positions:
        distances[pos][pos] = 0  # Distance from a position to itself is 0
        bfs(pos)

    # Accumulate distances between all pairs
    total_distance = 0
    unique_pairs = []
    for pos1 in distances:
        for pos2 in distances[pos1]:
            if pos1 != pos2 and (pos2, pos1) not in unique_pairs:
                unique_pairs.append((pos1, pos2))
                total_distance += distances[pos1][pos2]

    return total_distance

exp_grid = parse_input(run_str)
total_distances = shortest_path(exp_grid, 2)
print(f'Part 1 Answer: {total_distances}')

# Part 2:
new_grid = parse_input(run_str)

total_distances = shortest_path(new_grid, 1000000)
print(f"Part 2 Answer: {total_distances}")
