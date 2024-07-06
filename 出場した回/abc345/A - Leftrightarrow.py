s = input()
k = s[1:-2]
print(k)

if s[0] == "<" and s[-1] == ">" and "<" not in k and ">" not in k:
  print("Yes")
else:
  print("No")
