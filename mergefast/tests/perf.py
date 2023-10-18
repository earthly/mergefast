from timeit import timeit
import random
import matplotlib.pyplot as plt
import textwrap
import mergefast

def merge_latin(a,b):
   m1 = mergefast.merge_latin(a, b)

def merge_simple(a,b):
   m1 = mergefast.merge(a, b)

def sort(a,b):
   m2 = a + b 
   m2.sort()

num_data_points = 200
step = 100
methods = [
    "sort(a,b)",
    "merge_simple(a,b)",
    "merge_latin(a,b)"
]
labels = [
    "list(a+b).sort()",
    "mergefast.merge(a,b) #general",
    "mergefast.merge_latin(a,b) #specialized"
]

x = list(range(0, num_data_points * step, step))
y = [[] for _ in methods]
for i in x:
    list_a = random.sample(range(1, 100000000), i) + [0.1]
    list_b = random.sample(range(1, 100000000), int(i/4)) + [0.1]
    list_a.sort()
    list_b.sort()
    list_a = [str(x) for x in list_a]
    list_b = [str(x) for x in list_b]
    setup = textwrap.dedent(f"""\
        from __main__ import merge_latin, merge_simple, sort
        a = {list_a}; b = {list_b}
        """)
    for method_index, method in enumerate(methods):
        y[method_index].append(timeit(method, setup=setup, number=30))
    print(i, "out of", num_data_points * step)

ax = plt.axes()
for method_index, method in enumerate(labels):
    ax.plot(x, y[method_index], label=method)
ax.set(xlabel="number of elements lists", ylabel="time (s) (lower is better)")
ax.legend()
plt.savefig('perf.png')
