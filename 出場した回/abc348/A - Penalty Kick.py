n = int(input())
k =""

for i in range(n):
  if (i+1)%3==0:
    k+="x"
  else:
    k+="o"

print(k)