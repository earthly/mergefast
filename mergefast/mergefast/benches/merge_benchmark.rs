// use pyo3::Python;
// use pyo3::types::PyList;
// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use mergefast::merge;
// use mergefast::fibonacci;


// pub fn merge_benchmark(c: &mut Criterion) {
//     c.bench_function("merge", |b| {
//         b.iter(|| {
//             // Set up your benchmark here. For example:
//             let py = Python::acquire_gil();
//             let py = py.python();
//             let list1 = PyList::new(py, &[1, 3, 5]); // Example data
//             let list2 = PyList::new(py, &[2, 4, 6]); // Example data

//             // Call your function with black_box to prevent compiler optimizations
//             let _ = merge(py, list1, list2);
//         });
//     });
// }

// criterion_group!(benches, merge_benchmark);
// criterion_main!(benches);


// // pub fn criterion_benchmark(c: &mut Criterion) {
// //     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// // }

// // criterion_group!(benches, criterion_benchmark);
// // criterion_main!(benches);
