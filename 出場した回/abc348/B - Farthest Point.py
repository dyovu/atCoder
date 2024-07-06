n = int(input())
d = [[int(x) for x in input().split()] for _ in range(n)]
dis=[0.0]*n
poi=[0]*n

for i in range(len(d)):
  # print("A:",i)
  for j in range(len(d)-i-1):
    # print("B:",i+j+1)
    k = ((d[i][0]-d[i+j+1][0])**2 + (d[i][1]-d[i+j+1][1])**2)**(1/2)
    if dis[i] < k:
      dis[i] = k
      poi[i] = i+j+2
    if dis[i+j+1] < k:
      dis[i+j+1] = k
      poi[i+j+1] = i+1

# print(dis)
for i in poi:
  print(i)