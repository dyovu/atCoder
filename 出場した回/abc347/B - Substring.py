s = input()
t = len(s)
b =set()

for i in range(t):
  for j in range(t+1):
    b.add(s[i:j])

print(len(b)-1)