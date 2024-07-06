n, m = [int(x) for x in input().split()]

s = [input() for _ in range(n)]
taK_code = ['###.','###.','###.','....','....','.###','.###','.###']
list_2d =[]
ans =[]

for i in range(n-8):
    for j in range(m-8):
        list_2d = []
        for k in range(4):
            list_2d.append(s[i+k][j:j+4])
        for k in range(4):
            list_2d.append(s[i+k+5][j+5:j+9])
        if list_2d == taK_code:
            ans.append([i+1,j+1])

for i in ans:
    print(*i)