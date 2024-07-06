import math

n =int(input())
num_list =[n]
val = {1:[0,0], 2:[2,0]}
sumval =0

while(len(num_list)>0):
  # print(num_list)
  k =num_list.pop()
  # print(k)

  ku = math.floor(k/2)
  kd = math.ceil(k/2)

  if k in val:
    val[k][1] += 1
    # print(val)
  elif ku in val and kd in val:
    val[k] =[]
    val[k].append(val[ku][0] + val[kd][0] + k)
    val[k].append(1)
    # print(val)
  else:
    if ku <2:
      continue
    else:
      num_list.append(ku)
    if kd <2:
      continue
    else:
      num_list.append(kd)
      
    sumval += k
  
# print(val)

val_list = list(val.values())
# print(val_list)

for i in val_list:
  sumval += i[0]*i[1]

print(sumval)

