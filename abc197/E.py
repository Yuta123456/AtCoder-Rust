n = int(input())
ball = []
color = set()
for i in range(n):
    ball.append(list(map(int, input().split())))
    color.add(ball[i][1])
ball.append([0, n+1])
color.add(n+1)
color = list(color)
color.sort()
ball.sort(key=lambda x: x[1])
next_color = {color[i]: color[i+1] for i in range(len(color)-1)}
next_color[color[-1]] = n+1
first_color = color[0]
print(ball)
dic = {}
for i in range(n+1):
    x, c = ball[i]
    if c not in dic:
        dic[c] = []
    dic[c].append((x,i))
adjacent_list = [[] for i in range(n+1)]
for i in range(n+1):
    x, c = ball[i]
    # 同じ色への辺
    for n_x, node_num in dic[c]:
        # 自分は除く
        if node_num == i:
            continue
        adjacent_list[i].append((node_num, abs(n_x - x)))
    # 次の色への辺
    for n_x, node_num in dic[next_color[c]]:
        adjacent_list[i].append((node_num, abs(n_x - x)))

from heapq import heappush, heappop
def dijkstra(start, graph):
    INF = 10 ** 15
    dist = [INF] * (n+1)
    dist[start] = 0
    q = [(0,start)]
    while q:
        d,v = heappop(q)
        if dist[v] < d:
            continue
        for w,a in graph[v]:
            d1 = d + a
            if dist[w] > d1:
                dist[w] = d1
                heappush(q, (d1,w))
    return dist
print(adjacent_list)
dist = dijkstra(0, adjacent_list)
print(dist)