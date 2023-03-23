from math import *

plot = ['X', 'Y']
a = [int(input('Введите ' + str(plot[i])+': ')) for i in range(2)]

print(
    f'Вы ввели следующие числа: X = {a[0]}; Y = {a[1]}')
print('Ответ:', pow(1-(tan(a[0])), 1/tan(a[0])) + cos(a[0]-a[1]))
