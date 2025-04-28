x = input()

if len(x) == 1:
  y = 0
else:
  y = x[:-1]

if y == "-":
  yi = 0
else:
  yi = int(y)


if x[-1] == "0":
  print(y)
elif y == "-":
  print(0)
elif yi < 0:
  print(yi)
else:
  print(yi+1)
