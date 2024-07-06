q = int(input())
a =[]
op =[]

query = [list([int(x) for x in input().split()]) for _ in range(q)]
# print(query)

for i in range(q):
  if query[i][0]==1:
    a.append(query[i][1])
  else:
    op.append(a[len(a)-query[i][1]])

# print(a)
# print(op)

for i in op:
  print(i)