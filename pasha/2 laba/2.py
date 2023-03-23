a = input('Введите число : ')
min = int(a[0])
max = int(a[0])

for i in a:
    if int(i) > max:
        max = int(i)
    if int(i) < min:
        min = int(i)

print(max, '- большее число')
print(min, '- меньшее число')
