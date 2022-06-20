n = int(input())
a = list(map(int, input().split()))

ans = 10**18
for i in range(2**len(a)):
    binary = bin(i)[2:].zfill(len(a))
    # print(binary)
    sep = []
    for j in range(len(binary)):
        if binary[j] == '1':
            sep.append(j)
    group = []
    pre = 0
    for j in sep:
        group.append(a[pre:j])
        pre = j
    group.append(a[pre:])
    or_list = []
    for j in group:
        or_res = 0
        for k in j:
            or_res |= k
        or_list.append(or_res)
    xor_res = 0
    for j in or_list:
        xor_res ^= j
    ans = min(ans, xor_res)
print(ans)

