num = str(input('Введите символы : '))[1:]
new_num = ''
c = ''
i = 0
while num[i] != '.':
    if num[i] == ' ':
        num = num[:i+1]+num[i+2:]
    i += 1

print(num)
