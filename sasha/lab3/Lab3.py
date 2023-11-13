import turtle
import time
import math

class block():
    def __init__(self, x=0, y=0):
        self.x = x
        self.y = y
        self.line = turtle.Turtle() #создание обекта класса черепашка
        self.line.pensize(3) #задание размера линии
        self.line.color('red') #задание цвета линии
        #self.line.hideturtle() #отключение отображения рисовалки

    def draw(self, x, y):
        self.x = x
        self.y = y
        self.line.clear()  # стирание нарисованного
        self.line.up()
        self.line.goto(x - 5, y - 5)
        self.line.down()
        self.line.goto(x + 5, y - 5)
        self.line.goto(x + 5, y + 5)
        self.line.goto(x - 5, y + 5)
        self.line.goto(x - 5, y - 5)

class border:

    def __init__(self, lenght , width, x = 0, y = 0):
        self.x = x
        self.y = y
        self.lenght = lenght
        self.width = width
        self.left_edge = x - lenght / 2
        self.right_edge = x + lenght / 2
        self.down_edge = y - lenght / 2
        self.up_edge = y + lenght / 2
        self.line = turtle.Turtle() #создание обекта класса черепашка
        self.line.pensize(3) #задание размера линии
        self.line.color('black') #задание цвета линии
        #self.line.hideturtle() #отключение отображения рисовалки

    def draw(self):
        x = self.x
        y = self.y
        lenght = self.lenght
        width = self.width
        left_edge = self.left_edge
        right_edge = self.right_edge
        down_edge = self.down_edge
        up_edge = self.up_edge
        self.line.clear()  # стирание нарисованного
        self.line.up()
        self.line.goto(left_edge, down_edge)
        self.line.down()
        self.line.goto(left_edge, up_edge)
        self.line.goto(right_edge, up_edge)
        self.line.goto(right_edge, down_edge)
        self.line.goto(left_edge, down_edge)

    def is_border(self, x, y):
        left_edge = self.left_edge
        right_edge = self.right_edge
        down_edge = self.down_edge
        up_edge = self.up_edge
        if x <= left_edge:
            return 'left'
        elif x >= right_edge:
            return 'right'
        elif y <= down_edge:
            return 'down'
        elif y >= up_edge:
            return 'up'

window = turtle.Screen() #создание обекта класса окно
window.tracer(0,0) #отключение анимации рисования
block_1 = block()
border_1 = border(200, 100)
border_1.draw()
dt = 0.04
t_lim = 1000
N = int(t_lim/dt)
m = 0.1 # масса шарика в кг
k = 10 # массштабирующий каофициент в пиксель/метр
k_air = 0.7 # каофициент трения воздуха
g = 9.8 * k # ускорение свободного падения в пиксель/c^2
v_x = 100 # скорость в пикселях по оси х
v_y = 60 # скорость в пикселях по оси y
v_wind = 10 # cкорость ветра x
a_x = 0 # ускорение в пикселях по оси х
a_y = 0 # ускорение в пикселях по оис у
F_x = 0 # сила действующая по оси х
F_y = -m * g # сила действующая по оси y

for i in range(N):
    t_start = time.time()
    x = block_1.x + v_x * dt #Равномерное движение по оси x
    y = block_1.y + v_y * dt #Равномерное движение по оси y
    block_1.draw(x, y)
    window.update() # обновление нарисованного
    F_x += -k_air*(v_x - v_wind)
    a_x = F_x / m * k
    a_y = F_y / m * k
    v_x += a_x * dt
    v_y += a_y * dt
    if border_1.is_border(x, y) == 'right' or border_1.is_border(x, y) == 'left':
        v_x = -0.5 * v_x
    elif border_1.is_border(x, y) == 'up' or border_1.is_border(x, y) == 'down':
        v_y = -0.5 * v_y
    while time.time() - t_start < dt:
        pass
turtle.done()