from math import sqrt

plot = ['X', 'Y', 'Z']
a = [int(input('Введите ' + str(plot[i])+': ')) for i in range(3)]

print(
    f'Вы ввели следующие числа: X = {a[0]}; Y = {a[1]}; Z = {a[2]}')
print('Длина вектора:', sqrt(pow(a[0], 2)+pow(a[1], 2)+pow(a[2], 2)))
