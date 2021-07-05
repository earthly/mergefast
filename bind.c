#include "merge.h"

PyMethodDef merge_funcs[] = { {
								  "merge", /* function name */
								  (PyCFunction)merge,
								  METH_VARARGS,
								  "merge two sorted lists" /* function docs */
							  },
							  { NULL } };

char helloworldmod_docs[] = "This is hello world module.";

PyModuleDef merge_mod = { PyModuleDef_HEAD_INIT,
						  "merge", /* library name */
						  "merge module", /* module docs */
						  -1,
						  merge_funcs,
						  NULL,
						  NULL,
						  NULL,
						  NULL };

PyMODINIT_FUNC PyInit_merge( void )
{
	return PyModule_Create( &merge_mod );
}
