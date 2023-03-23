from matplotlib import pyplot as plt
from math import *
import numpy as np

if __name__ == '__main__':
    X = np.zeros(300)
    Y = np.zeros(300)

    j = 2*pi/300

    for b in range(150):
        X[b] = sin(j*b)
        Y[b] = cos(3*(j*b)) / 2

    for i in range(150, 300):
        X[i] = sin(i*j) / 2
        Y[i] = cos(i*j) / 2

    plt.plot(X, Y)
    plt.grid()
    plt.show()
