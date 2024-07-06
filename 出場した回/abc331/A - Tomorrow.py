M, D = [int(x) for x in input().split()]
y, m, d = [int(x) for x in input().split()]

tmrw_d = d + 1
tmrw_m = m
tmrw_y = y

if tmrw_d > D:
    tmrw_d = 1
    tmrw_m += 1

if tmrw_m > M:
    tmrw_m = 1
    tmrw_y += 1

print(tmrw_y)
print(tmrw_m)
print(tmrw_d)

