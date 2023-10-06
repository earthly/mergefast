import pymerge
import timeit

a = list(range(-100, 1700)) + [0.1]
b = list(range(1400, 1800))

def merge_int_test():
   m1 = pymerge.merge_int(a, b)

def merge_test():
   m1 = pymerge.merge(a, b)

def sort_test():
   m2 = list(a + b)
   m2.sort()

sort_time = timeit.timeit("sort_test()", setup="from __main__ import sort_test", number=100000)
merge_time = timeit.timeit("merge_test()", setup="from __main__ import merge_test",number=100000)
merge_int_time = timeit.timeit("merge_int_test()", setup="from __main__ import merge_int_test",number=100000)


print(f'timsort took {sort_time} seconds')
print(f'merge_int took {merge_int_time} seconds')
print(f'merge took {merge_time} seconds')
