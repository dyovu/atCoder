n = int(input())

list_2d = [list(map(int, input().split())) for _ in range(n)]
suc = {}
suc2_keys = []

for i in range(n):
    suc[i] = list_2d[i][0]/(list_2d[i][0] +list_2d[i][1])

print(suc)

suc2 = sorted(suc.items(), key=lambda x:x[1], reverse=True)

print(suc2)

for i in suc2:
    suc2_keys.append(i[0]+1)

print(suc2_keys)


suc2_keys = map(str, suc2_keys)

print(" ".join(suc2_keys))