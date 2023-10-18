# Py Merge

Merge sorted list faster than using `list.sort` or `heapq.merge`.

``` python
import mergefast

# create some sorted lists
a = list(range(-100, 1700))
b = list(range(1400, 1800))

# merge them
merged = mergefast.merge(a, b)
```

Also specialized variants for even faster merging of homogenus lists:

``` python
# Fast merge ints
mergefast.merge_int(a,b)

# Fast merge latin strings
mergefast.merge_latin(c,d)

# Fast merge floats
mergefast.merge_float(e,f)
```

See [article](https://earthly.dev/blog/python-timsort-merge)

## Building

To build the extension run `earthly +build`. To Test run `earthly +test` and to geneartor a performance graph run `earthly +perf`
