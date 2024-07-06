n, t = [int(x) for x in input().split()]
matrix = [[int(x) for x in input().split()] for _ in range(t)]

score = [0]*n
kinds = set(score)

for i in matrix:
  score[i[0]-1] += i[1]
  kinds =set(score)
  # print(score)
  print(len(kinds))