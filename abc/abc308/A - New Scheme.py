import sys

l = [int(x) for x in input().split()]
m = 100
n = 675

for i in range(8):
  if m<=l[i] and l[i]%25==0:
    m = l[i]
    if i==7 and l[7]<=n:
      print("Yes")
      sys.exit()
  else:
    break
      
print("No")