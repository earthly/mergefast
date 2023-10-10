#!/usr/bin/env python3

from setuptools import setup, Extension

merge_module = Extension(
    "mergefast.mergefast",
    sources=["mergefast/bind.c", "mergefast/merge.c"],
    extra_compile_args=["-O3"]
)

setup(
    name="mergefast",
    version="1.1.1",
    ext_modules=[merge_module],
)
