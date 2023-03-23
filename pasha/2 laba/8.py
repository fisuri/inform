num = str(input('Введите число : '))
new_num = ''
num = sorted(num)

for i in num:
    new_num += str(i)

print(new_num[::-1])
