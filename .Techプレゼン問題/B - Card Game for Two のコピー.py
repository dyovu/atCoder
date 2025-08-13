n = int(input())
card = [int(x) for x in input().split()]
a_points = 0
b_points = 0

#最初にソートすれば簡単
sorted_card = sorted(card ,reverse=True)
# print(sorted_card)

for i in range(len(sorted_card)):
    if i%2==0:
        a_points += sorted_card[i]
    else:
        b_points += sorted_card[i]

diff = a_points - b_points
print(diff)
