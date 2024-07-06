import math
c = int(input())

n=1
m=1
def formula(k):
  return k*(k+1)*(2*k+1)/12+k*(k+1)/4

def formula2(k):
  return k*(k+1)/2

while formula(n)<c:
  n+=1

second_digit = c-formula(n-1)

while formula2(m)<second_digit:
  m+=1

l = int(second_digit-formula2(m-1))

a1 = "1"*n
a2 = "1"*m
a3 = "1"*l

sum = int(a1)+int(a2)+int(a3)

print(sum)