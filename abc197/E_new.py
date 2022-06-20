n = int(input())
ball = {}
INF = 10**18

for i in range(n):
    b = list(map(int, input().split()))
    if b[1] not in ball:
        ball[b[1]] = []
    ball[b[1]].append(b[0])
max_color = len(ball.keys()) + 1
# 始点
ball[0] = [0]
# 終点
ball[n+1] = [0]
for k in ball.keys():
    ball[k].sort()
dp_r = [INF for i in range(n+2)]
dp_l = [INF for i in range(n+2)]
dp_r[0] = 0
dp_l[0] = 0
keys = list(ball.keys())
keys.sort()
for i in range(len(keys)-1):
    # 今回の端っこの位置
    color = keys[i]
    next_color = keys[i+1]
    l, r = ball[color][0], ball[color][-1]
    # 次の端っこの位置
    n_l, n_r = ball[next_color][0], ball[next_color][-1]
    # 次の右端の位置を決める
    dp_r[i+1] = min( \
        # 右端から、次の左を通って右に戻る
        dp_r[i] + abs(r - n_l) + (n_r - n_l),
        # 左端から、次の左を通って右に戻る
        dp_l[i] + abs(l - n_l) + (n_r - n_l))
    
    # 次の左端の位置を決める
    dp_l[i+1] = min(
        # 右端から、次の右を通って左に戻る
        dp_r[i] + abs(r - n_r) + (n_r - n_l),
        # 左端から、次の右を通って左に戻る
        dp_l[i] + abs(l - n_r) + (n_r - n_l))

print(min(dp_l[max_color], dp_r[max_color]))
# print(dp_l, dp_r)
