n, m = [int(x) for x in input().split()]
candi = [int(x) for x in input().split()]

vote_num = {n+1: 0 for n in range(n)}
max_voted = 1

for i in  candi:
    vote_num[i] += 1
    if vote_num[max_voted]<vote_num[i] or (vote_num[max_voted]==vote_num[i] and i<max_voted):
        print(i)
        max_voted = i
    else:
        print(max_voted)

        
# print(vote_num)