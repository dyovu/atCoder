n = int(input())
letter = [input() for _ in range(n)]

fin_letter = []
k = 0

for i in range(len(letter)):
    fin_letter += [letter[i]]
    for j in range(len(fin_letter)-1):
        if letter[i] == fin_letter[j] or letter[i] == fin_letter[j][::-1]:
            k += 1
            break

print(n-k)