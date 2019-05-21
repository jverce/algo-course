def find_max(a, n_cmp=0):
    n = len(a)
    if n == 1:
        return (a[0], n_cmp)
    else:
        split = int(n / 2)
        (ml, n_cmp_l) = find_max(a[:split], n_cmp)
        (mr, n_cmp_r) = find_max(a[split:], n_cmp)
        n_cmp = n_cmp_l + n_cmp_r + 1
        return (max(ml, mr), n_cmp)

def find_second_max(a):
    n = len(a)
    if n == 1:
        return None
    else:
        split = int(n / 2)
        ml = find_max(a[:split])
        mr = find_max(a[split:])
        return min(ml, mr)
