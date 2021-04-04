def exp(x, y):
    if y <= 0:
        return 1
    
    return x * exp(x, y - 1)

print(exp(2, 32))