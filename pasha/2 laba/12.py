a = input()
count = 1
temp_count = 1
for i in range(len(a)):
    if a[i] == a[i-1]:
        temp_count += 1
    elif count < temp_count:
        count = temp_count
        temp_count = 1
    else:
        temp_count = 1
print(count)
