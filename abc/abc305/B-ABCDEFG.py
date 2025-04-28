s = input().split()
dist = {"AB":3,"BC":1,"CD":4,"DE":1,"EF":5,"FG":9}
d = 23

for i in dist:
    if s[0] == i[0] or s[1] == i[0]:
        break
    else:
        d = d - dist[i]

rev_dict = sorted(dist.items(), reverse=True)

for i in rev_dict:
    if s[0] == i[0][1] or s[1] == i[0][1]:
        break
    else:
        d = d - i[1]

print(d)