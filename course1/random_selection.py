from partition import partition

def random_selection(a, i, start=None, end=None):
    start = start if start is not None else 0
    end = end if end is not None else len(a)
    n = end - start
    if n == 1:
        return a[start]
    else:
        pivot_pos = partition(a, start, end)
        if pivot_pos == i:
            return a[pivot_pos]
        elif pivot_pos < i:
            return random_selection(a, i, pivot_pos + 1, end)
        else:
            return random_selection(a, i, start, pivot_pos)
