def sieve_of_eratosthenes(n):
    primes = [True]*(n+1)
    primes[0] = primes[1] = False

    for p in range(2, int(n**0.5)+1):
        if primes[p]:
            primes[p**2:n+1:p] = [False] * len(primes[p**2:n+1:p])

    return [i for i in range(n+1) if primes[i]]


n = int(input("Введите число N: "))
primes = sieve_of_eratosthenes(n)
print("Простые числа на интервале [0;{}]:".format(n))
print(primes)
