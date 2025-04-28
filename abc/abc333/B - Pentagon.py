s = input()
t = input()

dic = {"A":1, "B":2, "C":3, "D":4, "E":5}

diff_s = abs(dic[s[0]]-dic[s[1]])

diff_t = abs(dic[t[0]]-dic[t[1]])

if (diff_t == diff_s or diff_s+diff_t == 5):
  print("Yes")
else:
  print("No")