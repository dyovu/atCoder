import math

n = int(input())
n_str = str(n)
half_len = int(len(n_str)/2)

cubic = math.pow(n, 1/3)



#一番近い回文に変換、一回の処理
for i in range(half_len):
  l =  int(n_str[-(i+1)]) - int(n_str[i])
  print(l)
  n = n-l*10**i
  print(n)
