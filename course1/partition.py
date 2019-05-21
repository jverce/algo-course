from statistics import median

def partition(a, start=None, end=None, pivot_pos=None):
    start = start if start is not None else 0
    end = end if end is not None else len(a)
    pivot_pos = pivot_pos if pivot_pos is not None else start
    n = end - start
    if n <= 1:
        return a
    i = start + 1

    # mid = start + (int(n/2) if n % 2 != 0 else int(n/2) - 1)
    # candidates = [
    #     (a[start], start),
    #     (a[mid], mid),
    #     (a[end-1], end-1),
    # ]
    # pivot_pos = median(candidates)[1]
    # pivot_pos = start
    # pivot_pos = end - 1

    pivot = a[pivot_pos]
    a[start], a[pivot_pos] = a[pivot_pos], a[start]
    for j in range(start+1, end):
        if a[j] < pivot:
            a[i], a[j] = a[j], a[i]
            i += 1
    a[start], a[i-1] = a[i-1], a[start]
    return i - 1
