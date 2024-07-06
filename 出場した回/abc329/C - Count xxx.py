n = int(input())
s = input()
s = s + " "

former_string = ""
string_len = {}
count = 0
sum = 0

for i in s:
    if former_string == "" or i == former_string:
        count += 1
        former_string = i
    else:
        if former_string in string_len and string_len[former_string]>count:
            count = 1
            former_string = i
            # print(string_len)
        else:
            string_len[former_string] = count
            count = 1
            former_string = i
            # print(string_len)

for i in string_len.values():
    sum += i

print(sum)

# print(string_len)