n, a, b = [int(x) for x in input().split()]
d = [int(x) for x in input().split()]
flag = True
rem = []

for i in d:
  rem.append(i%(a+b))
  # if i%(a+b) <= a:
  #   rem.append(i%(a+b))
  #   continue
  # else:
  #   print("No")
  #   flag = False
  #   break

if max(rem) - min(rem) <a:
  print("Yes")
else:
  print("No")

print(min(rem))
print(max(rem))
print(rem)