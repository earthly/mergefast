import mergeslow
import timeit

a = list(range(-100, 1700)) + [0.1]
b = list(range(1400, 1800))

def merge_test():
   m1 = mergeslow.merge(a, b)

def merge_exp_test():
   m1 = mergeslow.merge_with_exponential_search(a, b)

def sort_test():
   m2 = a + b
   m2.sort()

sort_time = timeit.timeit("sort_test()", setup="from __main__ import sort_test", number=100000)
merge_time = timeit.timeit("merge_test()", setup="from __main__ import merge_test",number=100000)
merge_exp_test_time = timeit.timeit("merge_exp_test()", setup="from __main__ import merge_exp_test",number=100000)

print(f'timsort took {sort_time} seconds')
print(f'mergeslow took {merge_time} seconds')
print(f'merge_exp_test took {merge_exp_test_time} seconds')
