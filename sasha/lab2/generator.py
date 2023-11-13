from math import sin, cos, pi


def ADC(t_lim, dt):
    N = int(t_lim / dt)
    t = [0 for i in range(N)]
    ADC_val = [0 for i in range(N)]
    for i in range(N):
        if i == 0:
            t[i] = 0
        else:
            t[i] = t[i - 1] + dt
        ADC_val[i] = sin(7*2*pi*t[i])+cos(16*2*pi*t[i])
    return t, ADC_val
