import matplotlib.pyplot as plt
import generator as gen

dt = 0.01    #  Период дискретизации сигнала
t_lim = 0.6   # Длительность считывания сигнала
bit = 16    # Разрядность АЦП
ideal_t, ideal_val = gen.ADC(t_lim, 0.00001)  #Получение эталонного сигнала от генератора

t, ADC_val = gen.ADC(t_lim,dt)               #Получение оцифроанного сигнала
ADC_val_integrated = [0 for i in range(len(ADC_val))]
ADC_val_diffrerentiated = [0 for i in range(len(ADC_val))]

for i in range(1,len(ADC_val_integrated)):
	#if i != len(ADC_val_integrated)-1:
	#	ADC_val_diffrerentiated[i] = (ADC_val[i+1] - ADC_val[i]) / dt
	ADC_val_diffrerentiated[i] = (ADC_val[i]-ADC_val[i-1])/dt
	ADC_val_integrated[i] = (ADC_val[i-1]+ADC_val[i])/2*dt+ADC_val_integrated[i-1]

print('Пройденный путь в t = 0.1: ', ADC_val_integrated[int(0.1/dt)])
print('Пройденный путь в t = 0.5: ', ADC_val_integrated[int(0.5/dt)])
print('Ускорение в t = 0.1: ', ADC_val_diffrerentiated[int(0.1/dt)])
print('Ускорение в t = 0.5: ', ADC_val_diffrerentiated[int(0.5/dt)])

plt.figure(figsize=(12, 9))
plt.subplot(3, 1, 1)
plt.title("Движение робота") # заголовок
plt.xlabel("Время (с)") # ось абсцисс
plt.ylabel("Скорость робота") # ось ординат
plt.xlim(0, t_lim) #ограничение размера графика по времени
plt.grid()      # включение отображение сетки
plt.plot(ideal_t, ideal_val, '--r')  # построение эталонного графика
plt.stem(t, ADC_val)  # построение точек оцифровки
plt.plot(t, ADC_val)  # построение оцифрованого графика

plt.subplot(3, 1, 2)
plt.xlabel("Время (с)") # ось абсцисс
plt.ylabel("Пройденный путь (м)") # ось ординат
plt.xlim(0, t_lim) #ограничение размера графика по времени
plt.grid()      # включение отображение сетки
plt.plot(t, ADC_val_integrated)  # построение интегрированного графика

plt.subplot(3, 1, 3)
plt.xlabel("Время (с)") # ось абсцисс
plt.ylabel("Ускорение (м/с2)") # ось ординат
plt.xlim(0, t_lim) #ограничение размера графика по времени
plt.grid()      # включение отображение сетки
plt.plot(t, ADC_val_diffrerentiated)  # построение дифференцированного графика

plt.show()
