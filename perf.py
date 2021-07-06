from timeit import timeit
import random
import matplotlib.pyplot as plt
import textwrap
import merge

def merge_int(a,b):
   m1 = merge.merge_int(a, b)

def merge_simple(a,b):
   m1 = merge.merge(a, b)

def sort(a,b):
   m2 = list(a + b)
   m2.sort()

num_data_points = 1000
step = 100
methods = [
    "sort(a,b)",
    "merge_simple(a,b)",
    "merge_int(a,b)"
]

x = list(range(0, num_data_points * step, step))
y = [[] for _ in methods]
for i in x:
    # list_a = list(range(i)) + [-3.4]
    # list_b = list(range(i)) + [3.12]
    list_a = random.sample(range(1, 100000000), i) #+ [0.1]
    list_b = random.sample(range(1, 100000000), int(i/3)) #+ [0.1]
    list_a.sort()
    list_b.sort()
    setup = textwrap.dedent(f"""\
        from __main__ import merge_int
        from __main__ import merge_simple
        from __main__ import sort
        a = {list_a}; b = {list_b}
        """)
    for method_index, method in enumerate(methods):
        y[method_index].append(timeit(method, setup=setup, number=30))
    print(i, "out of", num_data_points * step)

ax = plt.axes()
for method_index, method in enumerate(methods):
    ax.plot(x, y[method_index], label=method)
ax.set(xlabel="number of elements in both lists", ylabel="time (s) (lower is better)")
ax.legend()
plt.show()
