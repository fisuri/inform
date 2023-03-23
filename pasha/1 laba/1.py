from matplotlib import pyplot as plt
from math import *
import numpy as np

if __name__ == '__main__':
    X = np.zeros(300)
    Y = np.zeros(300)
    j = 2*pi/300
    for i in range(300):
        X[i] = cos(i*j) + cos(2*i*j) / 2
        Y[i] = sin(i*j) - sin(2*i*j) / 2
    plt.plot(X, Y)
    plt.show()
