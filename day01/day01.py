

# Part 1
print(max([sum(map(int, group.splitlines())) for group in open('input/day01.in').read().split('\n\n')]))

# Part 2

print(sum(sorted([sum(map(int, group.splitlines())) for group in open('input/day01.in').read().split('\n\n')], reverse=True)[0:3]))