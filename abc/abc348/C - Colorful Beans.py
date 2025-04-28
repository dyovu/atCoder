n = int(input())
d = [[int(x) for x in input().split()] for _ in range(n)]
dis = {}

k = sorted(d,reverse=True)
# print(sorted(d,reverse=True))

for i in range(len(k)):
  dis[k[i][1]] = k[i][0]

print(max(dis.values()))