n = int(input())
matrix = [[int(x) for x in input().split()] for _ in range(n)]

# print(matrix)

for i in matrix:
  k=1
  output =[]
  for j in i:
    if j==1:
      output += [k]
    k+=1
  print(*output)