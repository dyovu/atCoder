N = int(input())
A = 0

if N%5 == 0:
  A = N
elif N%5 >= 3:
  A = N + 5 - N%5
else:
  A = N - N%5

print(A)