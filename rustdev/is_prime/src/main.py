import math
import time

def is_prime(n):
    if n <= 1:
        return False

    if n == 2 or n == 3:
        return True

    if n % 2 == 0 or n % 3 == 0:
        return False

    sqrt_n = math.isqrt(n)

    i = 5
    while i <= sqrt_n:
        if n % i == 0 or n % (i + 2) == 0:
            return False
        i += 6

    return True

def main():
    start = time.time()

    number = 11000
    vec = [n for n in range(2, number + 1) if is_prime(n)]
    print(vec)

    end = time.time()
    print(f"Execution time: {(end - start) * 1000:.2f} ms")

if __name__ == '__main__':
    main()