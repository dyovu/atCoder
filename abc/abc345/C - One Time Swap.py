s = input()

same_letter = {}
a = {}
b = 0
c ={}
sum = int(len(s)*(len(s)-1)/2)

k = 0

for i in s:
  if  i in same_letter.keys():
    # same_letter[i] = same_letter[i] + [k]
    c[i] = c[i] + 1
    b = b + int(c[i]*(c[i]-1)/2) -a[i]
    a[i] = int(c[i]*(c[i]-1)/2)
  else:
    # same_letter[i] = [k]
    c[i] = 1
    a[i] = int(c[i]*(c[i]-1)/2)
  k += 1

# print(same_letter)
# print(sum)
# print(a)
# print(b)
if b != 0:
  print(sum-b+1)
else:
  print(sum)

