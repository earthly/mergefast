#include "merge.h"

PyMethodDef merge_funcs[] = { {
								  "merge", /* function name */
								  (PyCFunction)merge,
								  METH_VARARGS,
								  "merge two sorted lists" /* function docs */
							  },
							  {
								  "merge_latin", /* function name */
								  (PyCFunction)merge_latin,
								  METH_VARARGS,
								  "merge two sorted lists of latin strings" /* function docs */
							  },
							  {
								  "merge_int", /* function name */
								  (PyCFunction)merge_int,
								  METH_VARARGS,
								  "merge two sorted lists of integers" /* function docs */
							  },
							  {
								  "merge_float", /* function name */
								  (PyCFunction)merge_float,
								  METH_VARARGS,
								  "merge two sorted lists of floats" /* function docs */
							  },
							  { NULL } };

PyModuleDef merge_mod = { PyModuleDef_HEAD_INIT,
						  "core", /* library name */
						  "core module", /* module docs */
						  -1,
						  merge_funcs,
						  NULL,
						  NULL,
						  NULL,
						  NULL };

PyMODINIT_FUNC PyInit_core( void )
{
	return PyModule_Create( &merge_mod );
}
