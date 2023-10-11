#!/usr/bin/env python3

from setuptools import setup, Extension

merge_module = Extension(
    "mergefast.core",
    sources=["mergefast/bind.c", "mergefast/merge.c"],
    include_dirs=["mergefast"],
    extra_compile_args=["-O3"]
)

setup(
    name="mergefast",
    version="1.1.2",
    ext_modules=[merge_module],
)
