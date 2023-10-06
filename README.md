# Py Merge

Merge sorted list faster than using `list.sort` or `heapq.merge`.

``` Python
import merge

# create some sorted lists
a = list(range(-100, 1700))
b = list(range(1400, 1800))

# merge them
merged = merge.merge(a, b)
```

See [article](https://earthly.dev/blog/python-timsort-merge)

To build the extension run `earthly +build`. To Test run `earthly +test` and to geneartor a performance graph run `earthly +perf` 
