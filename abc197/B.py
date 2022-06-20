h, w, x, y = list(map(int, input().split()))
grid = []
for i in range(h):
    grid.append(list(input()))

x -= 1
y -= 1

ans = 1
for i in range(1, 1000):
    if y+i >= w or grid[x][y+i] == '#':
        break
    ans += 1
for i in range(1, 1000):
    if y-i < 0 or grid[x][y-i] == '#':
        break
    ans += 1
for i in range(1, 1000):
    if x+i >= h or grid[x+i][y] == '#':
        break
    ans += 1
for i in range(1, 1000):
    if x-i < 0 or grid[x-i][y] == '#':
        break
    ans += 1
print(ans)