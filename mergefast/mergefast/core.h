#ifndef __core_H__
#define __core_H__

#include <Python.h>

PyObject* merge( PyObject*, PyObject* );
PyObject* merge_latin( PyObject*, PyObject* );
PyObject* merge_int( PyObject*, PyObject* );
PyObject* merge_float( PyObject*, PyObject* );

#endif
