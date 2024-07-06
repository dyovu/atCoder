a, b, d = [int(x) for x in input().split()]
ns = [a]


for i in range(int((b-a)/d)):
  ns.append(ns[i]+d)

for i in ns:
  print(i)