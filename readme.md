## Idea

- use rust and py03 for mergefast
- then add galopping to make even faster 

## Problems

- can't use specilized compares for python types like int and string
- even without those, and timsort without those, we are still half the speed
- adding galooping seems to not make a difference at all 

## Investigate

- what is the slow part?
 - Can't tell, because can't profile. 
  - Dtrace just giving memory locations, need to try profiling on linux.


Idea:
https://pyo3.rs/v0.19.2/conversions/tables
Using Rust library types vs Python-native types
Using Rust library types as function arguments will incur a conversion cost compared to using the Python-native types. Using the Python-native types is almost zero-cost (they just require a type check similar to the Python builtin function isinstance()).

However, once that conversion cost has been paid, the Rust standard library types offer a number of benefits:

You can write functionality in native-speed Rust code (free of Python's runtime costs).
You get better interoperability with the rest of the Rust ecosystem.
You can use Python::allow_threads to release the Python GIL and let other Python threads make progress while your Rust code is executing.
You also benefit from stricter type checking. For example you can specify Vec<i32>, which will only accept a Python list containing integers. The Python-native equivalent, &PyList, would accept a Python list containing Python objects of any type.
For most PyO3 usage the conversion cost is worth paying to get these benefits. As always, if you're not sure it's worth it in your case, benchmark it!
