def digital_root(n):
    while n >= 10:
        n = sum(int(digit) for digit in str(n))
    return n


result = digital_root(int(input()))
print(result)
