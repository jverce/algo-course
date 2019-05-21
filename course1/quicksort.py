from partition import partition

def quicksort(a, start=None, end=None):
    start = start if start is not None else 0
    end = end if end is not None else len(a)
    n = end - start
    if n <= 1:
        return (a, 0)
    else:
        slice = partition(a, start, end)
        (a, cmp_count_l) = quicksort(a, start, slice)
        (a, cmp_count_r) = quicksort(a, slice+1, end)
        cmp_count = cmp_count_l + cmp_count_r + end - start - 1
        return (a, cmp_count)
