def index_eq_content(a, i=0):
    n = len(a)
    if n == 1:
        return a[0] == i
    else:
        split = int(n / 2)
        l = a[split-1] >= i-1 and index_eq_content(a[:split], i + split-1)
        r = a[split] <= i and index_eq_content(a[split:], i + split)
        return l or r
