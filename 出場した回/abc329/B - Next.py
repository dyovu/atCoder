n = int(input())

num = [int(x) for x in input().split()]


num_set = set(num)
num_list = list(num_set)
num_sorted = sorted(num_list)

print(num_sorted[len(num_sorted)-2])