
n = int(input())
a = [int(x) for x in input().split()]
passenger = 0
k = 0

min = 0
num = 0

for i in range(len(a)):
  num += a[i]
  if num <min:
    min = num

kkkk = sum(a)+ -(min)
print(kkkk)

  