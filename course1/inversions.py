def count_split_inversions(left, right):
    i, j = 0, 0
    n_left, n_right = len(left), len(right)
    c = [0] * (n_left + n_right)
    res = 0

    for k in range(n_left + n_right):
        if i == n_left:
            c[k] = right[j]
            j += 1
        elif j == n_right:
            c[k] = left[i]
            i += 1
        elif left[i] > right[j]:
            res += n_left - i
            c[k] = right[j]
            j += 1
        else:
            c[k] = left[i]
            i += 1
    return (c, res)

def count_inversions(items):
    n = len(items)
    if n <= 1:
        return (items, 0)
    else:
        slice = int(n / 2)
        (a, x) = count_inversions(items[:slice])
        (b, y) = count_inversions(items[slice:])
        (c, z) = count_split_inversions(a, b)
        return (c, x + y + z)
